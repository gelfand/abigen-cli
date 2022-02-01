use std::path::PathBuf;

use ethers::prelude::Abigen;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(long = "name")]
    /// The name of the contract.
    contract_name: String,
    #[structopt(long = "abi")]
    /// The path to the ABI file.
    abi: String,
    #[structopt(long = "path")]
    /// The path to the output file.
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let path = if args.path.is_dir() {
        args.path.join(args.contract_name.clone() + ".rs")
    } else {
        args.path.clone()
    };

    std::env::set_var(
        "CARGO_MANIFEST_DIR",
        args.path.canonicalize().unwrap().to_str().unwrap(),
    );

    Abigen::new(&args.contract_name, &args.abi)?
        .generate()?
        .write_to_file(&path.as_path())?;
    Ok(())
}
