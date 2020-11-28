use color_eyre::eyre::Result;
use dotenv::dotenv;
use structopt::StructOpt;

mod day;
mod init;
mod run;

use init::Init;
use run::Run;

#[derive(StructOpt)]
#[structopt(name = "Advent Of Code")]
enum Args {
    /// Download input file for given day
    Init(Init),
    /// Run code of the given day
    Run(Run),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv()?;
    let args = Args::from_args();
    match args {
        Args::Init(init) => init.initialize()?,
        Args::Run(run) => {
            let output = run.run()?;
            println!("{}", output)
        }
    }
    Ok(())
}
