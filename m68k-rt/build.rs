use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::env;

fn main() {
    
    /*  This code was used in cortex-m-rt to take the first bit of the target
     *  json filename and use it to determine which linker script snippet to
     *  inject.

    let mut target = env::var("TARGET").unwrap();
    
    // When using a custom target JSON, `$TARGET` contains the path to that JSON file. By
    // convention, these files are named after the actual target triple, eg.
    // `thumbv7m-customos-elf.json`, so we extract the file stem here to allow custom target specs.
    let path = Path::new(&target);
    if path.extension() == Some(OsStr::new("json")) {
        target = path
            .file_stem()
            .map_or(target.clone(), |stem| stem.to_str().unwrap().to_string());
    }
    
    */
    
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let link_x = include_bytes!("link.x.in");
    if env::var_os("CARGO_FEATURE_DEVICE").is_some() {
        let mut f = File::create(out.join("link.x")).unwrap();
        
        f.write_all(link_x).unwrap();

        // *IMPORTANT*: The weak aliases (i.e. `PROVIDED`) must come *after*
        // `EXTERN(__INTERRUPTS)`. Otherwise the linker will ignore user defined
        // interrupts and always populate the table with the weak aliases.
        writeln!(
            f,
            r#"
/* Provides weak aliases (cf. PROVIDED) for device specific interrupt handlers */
/* This will usually be provided by a device crate generated using svd2rust */
INCLUDE device.x"#
        )
        .unwrap();
        f
    } else {
        let mut f = File::create(out.join("link.x")).unwrap();
        f.write_all(link_x).unwrap();
        f
    };
    
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x.in");
    
}