use clap::{ Parser, Subcommand, ValueEnum, Args };

#[derive(Debug, Parser)]
#[command(name = "superbridge-wallet", about = "multichain-wallet")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Standard {
    ERC,
    SPL
}

// TODO: add tokens
#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Token {
    USDC
}

// generate wallet erc/spl, import wallet erc/spl, check token balance erc/spl, transfer
#[derive(Debug, Subcommand)]
enum Commands {
    // generate wallet
    #[command(arg_required_else_help = true)]
    Generate {
        #[arg(long = "std")]
        standard: Standard
    },
    // import wallet
    #[command(arg_required_else_help = true)]
    Import {
        #[arg(long = "std")]
        standard: Standard,
        // TODO: any other stuff required to import
    },
    // check balance
    #[command(arg_required_else_help = true)]
    Balance {
        #[arg(long = "std")] // TODO: add short = "std"
        standard: Standard,
        #[arg(short = 't', long = "tkn")]
        token: Token
        // TODO: any other stuff required to import
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
        amount: u64
        // TODO: any other stuff required to import
    }
}

fn main()  {
    let args = CLI::parse();
    
    match args.command {
        Commands::Generate { standard } => {
            todo!()
        }
        Commands::Import { standard } => {
            todo!()
        }
        Commands::Balance { standard, token } => {
            todo!()
        }
        Commands::Transfer { source, destination, token, amount } => {
            todo!()
        }
    }
}
