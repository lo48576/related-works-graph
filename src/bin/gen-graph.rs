//! Works graph builder.

use std::{
    fs::{self, File},
    io,
    path::PathBuf,
    process::{Command, Stdio},
};

use failure::{bail, format_err, Fallible};
use related_works_graph::types::Works;
use structopt::StructOpt;

/// CLI option.
#[derive(Debug, Clone, StructOpt)]
pub struct CliOpt {
    /// Works file
    #[structopt(parse(from_os_str))]
    works: PathBuf,
    /// Output type
    #[structopt(short = "t", default_value = "svg")]
    output_type: String,
    /// Output file
    #[structopt(short = "o", parse(from_os_str))]
    output_path: Option<PathBuf>,
}

fn main() -> Fallible<()> {
    let opt = CliOpt::from_args();
    let works_str = fs::read_to_string(&opt.works)?;
    let works: Works = toml::from_str(&works_str)?;
    works.validate()?;

    write_graph(&works, &opt)?;

    Ok(())
}

fn write_graph(works: &Works, opt: &CliOpt) -> Fallible<()> {
    match opt.output_type.as_ref() {
        "dot" => {
            write_graph_direct_dot(works, opt)?;
        }
        ty => {
            write_graph_through_dot(works, ty, opt)?;
        }
    }

    Ok(())
}

fn write_graph_direct_dot(works: &Works, opt: &CliOpt) -> Fallible<()> {
    match opt.output_path.as_ref() {
        Some(path) => works.write_graph(File::open(path)?)?,
        None => works.write_graph(io::stdout().lock())?,
    }

    Ok(())
}

fn write_graph_through_dot(works: &Works, ty: &str, opt: &CliOpt) -> Fallible<()> {
    let mut command = Command::new("dot");
    command.args(&["-T", ty]);
    if let Some(out_path) = opt.output_path.as_ref() {
        command.stdout(File::create(out_path)?);
    } else {
        command.stdout(Stdio::inherit());
    }
    command.stdin(Stdio::piped());

    let mut child = command.spawn()?;
    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| format_err!("Failed to get stdin handle of the child process"))?;
        works.write_graph(stdin)?;
    }
    let exit_status = child.wait()?;
    if !exit_status.success() {
        bail!("`dot` command execution failed: exit={:?}", exit_status);
    }

    Ok(())
}
