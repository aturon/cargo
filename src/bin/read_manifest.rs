use std::env;
use std::error::Error;

use cargo::core::{Package, Source};
use cargo::util::{CliResult, CliError, Config};
use cargo::util::important_paths::{find_root_manifest_for_cwd};
use cargo::sources::{PathSource};

#[derive(RustcDecodable)]
struct Options {
    flag_manifest_path: Option<String>,
    flag_color: Option<String>,
}

pub const USAGE: &'static str = "
Usage:
    cargo read-manifest [options]
    cargo read-manifest -h | --help

Options:
    -h, --help               Print this message
    -v, --verbose            Use verbose output
    --manifest-path PATH     Path to the manifest to compile
    --color WHEN             Coloring: auto, always, never
";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<Package>> {
    debug!("executing; cmd=cargo-read-manifest; args={:?}",
           env::args().collect::<Vec<_>>());
    try!(config.shell().set_color_config(options.flag_color.as_ref().map(|s| &s[..])));

    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));

    let mut source = try!(PathSource::for_path(root.parent().unwrap(), config).map_err(|e| {
        CliError::new(e.description(), 1)
    }));

    try!(source.update().map_err(|err| CliError::new(err.description(), 1)));

    source.root_package()
          .map(|pkg| Some(pkg))
          .map_err(|err| CliError::from_boxed(err, 1))
}
