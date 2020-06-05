use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let path = opts.path.unwrap_or(std::env::current_dir()?);

    println!("{}", ist::Listing::new(path)?);

    Ok(())
}
