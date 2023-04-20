use clap::{ Parser, Subcommand, ValueEnum, Args };

#[derive(Debug, Parser)]
#[command()] // TODO: add "name", "about"
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
        #[arg(short, long)] // TODO: add short = "std"
        standard: Standard
    },
    // import wallet
    #[command(arg_required_else_help = true)]
    Import {
        #[arg(short, long)] // TODO: add short = "std"
        standard: Standard,
        // TODO: any other stuff required to import
    },
    #[command(arg_required_else_help = true)]
    Balance {
        #[arg(short, long)] // TODO: add short = "std"
        standard: Standard,
        #[arg(short, long)]
        token: Token
        // TODO: any other stuff required to import
    },
    #[command(arg_required_else_help = true)]
    Transfer {
        #[arg(short, long)] // TODO: add short = "src"
        source: Standard,
        #[arg(short, long)] // TODO: add short = "dst"
        destination: Standard,
        #[arg(short, long)]
        token: Token,
        #[arg(short, long)] // TODO: add short = "amt"
        amount: u64
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
