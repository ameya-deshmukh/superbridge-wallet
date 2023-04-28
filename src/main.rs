use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::{
    network::constants::Network,
    util::bip32::{DerivationPath, ExtendedPrivKey},
    PublicKey,
};
use clap::{Args, Parser, Subcommand, ValueEnum};
use eth_keystore::encrypt_key;
use hdpath::{Purpose, StandardHDPath};
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::{lamports_to_sol, sol_to_lamports},
    pubkey::Pubkey,
    signature::{keypair_from_seed, read_keypair_file, write_keypair_file, Keypair},
    signer::Signer,
};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
const SERVER_URL: &str = "https://api.devnet.solana.com";

#[derive(Debug, Parser)]
#[command(name = "superbridge-wallet", about = "multichain-wallet")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Standard {
    ERC,
    SPL,
}

// TODO: add tokens
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Token {
    USDC,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Source {
    Generated,
    Imported,
}

// generate wallet erc/spl, import wallet erc/spl, check token balance erc/spl, transfer
#[derive(Debug, Subcommand)]
enum Commands {
    // generate wallet
    #[command(arg_required_else_help = true)] // TODO: add mnemonic & passphrase || add output file
    Generate {
        #[arg(long = "std")]
        standard: Standard,
    },
    // import wallet
    #[command(arg_required_else_help = true)]
    Import {
        #[arg(long = "std")]
        standard: Standard,
        #[arg(long, value_delimiter = ',')]
        mnemonic: Vec<String>, // TODO: any other stuff required to import
    },
    // check balance
    #[command(arg_required_else_help = true)]
    Balance {
        #[arg(long = "std")]
        standard: Standard,
        #[arg(short = 't', long = "tkn")]
        token: Token,
        #[arg(long = "src")]
        source: Source, // TODO: any other stuff required to import
    },
    // transfer tokens
    #[command(arg_required_else_help = true)]
    Transfer {
        #[arg(long = "src")]
        source: Standard,
        #[arg(long = "dst")]
        destination: Standard,
        #[arg(short = 't', long = "tkn")]
        token: Token,
        #[arg(long = "amt")]
        amount: u64, // TODO: any other stuff required to import
    },
}

fn get_extended_keypair(
    seed: &[u8],
    hd_path: &StandardHDPath,
) -> (ExtendedPrivKey, ExtendedPubKey) {
    let secp = Secp256k1::new();
    let pk = ExtendedPrivKey::new_master(Network::Bitcoin, seed)
        // we convert HD Path to bitcoin lib format (DerivationPath)
        .and_then(|k| k.derive_priv(&secp, &DerivationPath::from(hd_path)))
        .unwrap();
    let pubk = ExtendedPubKey::from_private(&secp, &pk);
    (pk, pubk)
}

fn generate_keypair_erc() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    println!("Mnemonic: {}", mnemonic);

    // save it as a keystore file
    let entropy = mnemonic.entropy();
    // println!("Entropy: {:?}", entropy); //128 bits for 12 words, 256 bits for 24 words

    let mut rng = rand::thread_rng();
    let dir = Path::new("./erc");
    let uuid = encrypt_key(&dir, &mut rng, entropy, "password_to_keystore").unwrap();
    //println!("File uuid: {}", uuid);

    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();

    let hd_path = StandardHDPath::new(Purpose::Pubkey, 60, 0, 0, 0);
    let (_pk, pubk) = get_extended_keypair(&seed_bytes, &hd_path);

    println!("Public key: {:?}", pubk.public_key);

    // let _eth_addr = extended_pubk_to_addr(&pubk);
}

fn import_wallet_erc(phrase: Vec<String>) {
    let phrase: &String = &phrase.join(" ");

    let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let entropy = mnemonic.entropy();
    // println!("Entropy: {:?}", entropy); //128 bits for 12 words, 256 bits for 24 words

    let mut rng = rand::thread_rng();
    let dir = Path::new("./erc");
    let uuid = encrypt_key(&dir, &mut rng, entropy, "password_to_keystore").unwrap();
    //println!("File uuid: {}", uuid);

    let seed = Seed::new(&mnemonic, ""); //128 hex chars = 512 bits
    let seed_bytes: &[u8] = seed.as_bytes();

    let hd_path = StandardHDPath::new(Purpose::Pubkey, 60, 0, 0, 0);
    let (_pk, pubk) = get_extended_keypair(&seed_bytes, &hd_path);

    println!("Public key: {:?}", pubk.public_key);

    // let _eth_addr = extended_pubk_to_addr(&pubk);
}

fn generate_keypair_spl() {
    let mnemonic_type: MnemonicType = MnemonicType::for_word_count(12).unwrap();
    let mnemonic: Mnemonic = Mnemonic::new(mnemonic_type, Language::English);

    let seed: Seed = Seed::new(&mnemonic, "");

    let keypair: solana_sdk::signature::Keypair = keypair_from_seed(seed.as_bytes()).unwrap();
    write_keypair_file(&keypair, "./spl/generated/keypair.json").unwrap();

    println!("Mnemonic: {:?}", mnemonic);
    println!("Public key: {}", &keypair.pubkey());
}

fn import_wallet_spl(phrase: Vec<String>) {
    let phrase: &String = &phrase.join(" ");

    let mnemonic: Mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    let seed: Seed = Seed::new(&mnemonic, "");

    let keypair: Keypair = keypair_from_seed(&seed.as_bytes()).unwrap();
    write_keypair_file(&keypair, "./spl/imported/keypair.json").unwrap();

    println!("Imported wallet with public key: {}", &keypair.pubkey());
}

fn get_balance_spl(address: &str, client: &RpcClient) {
    let pubkey: Pubkey = Pubkey::from_str(address).unwrap();
    let balance: u64 = client.get_balance(&pubkey).unwrap();

    println!("Balance for {}: {}", address, lamports_to_sol(balance));
}

fn main() {
    let args: CLI = CLI::parse();
    let client: RpcClient = RpcClient::new(SERVER_URL);

    match args.command {
        Commands::Generate { standard } => match standard {
            Standard::ERC => {
                generate_keypair_erc();
            }
            Standard::SPL => {
                generate_keypair_spl();
            }
        },
        Commands::Import { standard, mnemonic } => match standard {
            Standard::ERC => {
                import_wallet_erc(mnemonic);
            }
            Standard::SPL => {
                import_wallet_spl(mnemonic);
            }
        },
        Commands::Balance {
            standard,
            token,
            source,
        } => match source {
            Source::Generated => match &standard {
                Standard::ERC => {}
                Standard::SPL => {
                    let keypair: Keypair =
                        read_keypair_file("./spl/generated/keypair.json").unwrap();
                    get_balance_spl(&keypair.pubkey().to_string(), &client);
                }
            },
            Source::Imported => match &standard {
                Standard::ERC => {}
                Standard::SPL => {
                    let keypair: Keypair =
                        read_keypair_file("./spl/imported/keypair.json").unwrap();
                    get_balance_spl(&keypair.pubkey().to_string(), &client);
                }
            },
        },
        Commands::Transfer {
            source,
            destination,
            token,
            amount,
        } => {
            todo!()
        }
    }
}
