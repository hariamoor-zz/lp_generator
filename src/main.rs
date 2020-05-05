use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

use serde_xml_rs::from_reader;
use std::error::Error;

use std::io::Write;

mod resource;
use resource::Resource;

mod kdg;
use kdg::build_model;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Project 2 - Solving Hard Compiler Problems",
    about = "Simplified implementation of Dr. Uli Kremer's RSDG algorithm with Gurobi"
)]
struct Opt {
    /// Execution resource budget provided by user (integer or floating-point value)
    #[structopt(long)]
    budget: f64,

    /// Path to input XML file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    xml: PathBuf,

    /// Path to output LP file
    #[structopt(long)]
    #[structopt(parse(from_os_str))]
    app: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opt = Opt::from_args();
    let resource: Resource = from_reader(File::open(&opt.xml)?)?;

    opt.app.set_extension("lp");
    let mut output = File::create(&opt.app)?;

    output.write(build_model(resource, opt.budget).as_bytes())?;

    Ok(())
}
