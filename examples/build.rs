extern crate handlebars;
use std::collections::BTreeMap;
use handlebars::Handlebars;

use std::fs::File;
use std::io::Write;
use std::env;

static SO_FILE: &'static str = "libexamples";
#[cfg(any(target_os = "linux", target_os = "android"))]
static SO_EXT: &'static str = "so";
#[cfg(target_os = "macos")]
static SO_EXT: &'static str = "dylib";
#[cfg(target_os = "windows")]
static SO_EXT: &'static str = "dll";

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("t1", "src/manifest.ttl.in").unwrap();
    let out_path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/../target/debug/manifest.ttl";
    let mut out_file = File::create(out_path).unwrap();
    let this = format!("{}.{}", SO_FILE, SO_EXT);
    let mut data = BTreeMap::new();
    data.insert("so".to_string(), this.to_string());
    let bubu = handlebars.render("t1", &data).unwrap();
    out_file.write_all(bubu.as_bytes());
    println!("cargo:warning=Out:{}", handlebars.render("t1", &data).unwrap().replace("\n", " "));
}
