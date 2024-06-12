use clap::Parser;
use miette::Result;
use scaffops::app::App;

fn main() -> Result<()> {
    App::parse().run()?;

    Ok(())
}
