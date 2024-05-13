//! An Example of using ASN.1 compiler in `build.rs`
//!

use std::env;
use std::path::PathBuf;

use asn1_compiler::{
    generator::{Codec, Derive, Visibility},
    Asn1Compiler,
};

fn main() -> std::io::Result<()> {
    println!("cargo::rerun-if-changed=specs/ngap");
    let module = "ngap_generated.rs";
    let spec_file_name: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("specs")
        .join("ngap")
        .join("ngap.asn");
    let spec_files = vec![spec_file_name];
    let rs_module = PathBuf::from(env::var("OUT_DIR").unwrap()).join(module);
    let rs_module = rs_module.to_str().unwrap();

    let mut compiler = Asn1Compiler::new(
        rs_module,
        &Visibility::Public,
        vec![Codec::Aper],
        vec![Derive::Debug],
    );

    compiler.compile_files(&spec_files)?;

    Ok(())
}
