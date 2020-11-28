use color_eyre::eyre::Result;
use structopt::StructOpt;

mod config;
mod day;
mod init;
mod run;

use config::Config;
use init::Init;
use run::Run;

#[derive(StructOpt)]
#[structopt(name = "Advent Of Code 2020")]
enum Args {
    /// Set configuration such as session key and input files
    Config(Config),
    /// Download input file for given day
    Init(Init),
    /// Run code of the given day
    Run(Run),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::from_args();
    match args {
        Args::Config(config) => config.save()?,
        Args::Init(init) => init.initialize()?,
        Args::Run(run) => {
            let output = run.run()?;
            println!("{}", output)
        }
    }
    Ok(())
}
