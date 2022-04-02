use terra_rust_bot_output::output::pretty::*;
 
use env_logger::Env; 
use structopt::StructOpt; 


#[derive(StructOpt)]
#[structopt(about = "a basic signal CLI to try things out")]
struct Args { 
    #[structopt(flatten)]
    subcommand: Subcommand,
}

#[derive(StructOpt)]
enum Subcommand {
    #[structopt(about = "Terra-rust-bot feature: print information directly to the console.")]
    LocalDisplay {
        #[structopt(long, short = "m", help = "message")]
        message: String,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::from_env(
        Env::default().default_filter_or(format!("{}=info", env!("CARGO_PKG_NAME"))),
    )
    .init();

    let args = Args::from_args();
  
    match args.subcommand {
 
        Subcommand::LocalDisplay {message} => {
            println!("{esc}c", esc = 27 as char); 
            println!("{}", terra_rust_bot_state(&message,"./terra-rust-bot-state.json").await);
        }  
    };
    Ok(())
}
