// Allow process::exit to keep the example simple
#![allow(clippy::exit)]

use std::{env, process};

use host_func_abi::GetVersionReq;
use semver::{Version, VersionReq};
use wasm_plugins::runtime::{get_func_complex_without_args, InstanceBuilder};
use wasmer::{imports, Function, Store};

type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

const RUNNER_VERSION: &str = "0.1.0";

fn main() {
    let mut args = env::args();
    let path = if let Some(path) = args.nth(1) {
        path
    } else {
        println!("Usage: ./simple-runner PLUGIN_PATH");
        process::exit(1);
    };

    if let Err(e) = run_plugin(&path) {
        eprintln!("Error when running the plugin: {}", e);
        process::exit(2);
    };
}

fn get_page(url: String) -> String {
    println!("Fetching {}", url);
    "It work".to_string()
}

fn run_plugin(path: &str) -> StdResult<()> {
    let store = Store::default();
    let get_page = Function::new_native(&store, |args: i32| {
        println!("IT WORK {}", args);
        args - 1
    });

    let instance = InstanceBuilder::builder()
        .wasm_path(path)
        .import_object(imports! {
            "env" => {
                "get_page"  => get_page,
            }
        })
        .build()
        .finalize()?;

    dbg!(&instance.exports);

    let get_plugin_requirements = get_func_complex_without_args::<GetVersionReq>(&instance)?;

    let plugin_requirements = get_plugin_requirements()?;

    println!("Runner version is {}", RUNNER_VERSION);
    println!("Plugin requirements are {}", plugin_requirements);

    let semver_req = VersionReq::parse(&plugin_requirements)?;
    let runner_version = Version::parse(RUNNER_VERSION)?;

    if semver_req.matches(&runner_version) {
        println!("The runner support this plugin");
    } else {
        println!("The runner does not support this plugin");
    }

    Ok(())
}
