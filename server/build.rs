use std::{env, fs, path::Path};

fn main() {
    let out = env::var_os("OUT_DIR").unwrap();
    let picocss_path = env::var_os("PICO_PATH").unwrap();
    let syntaxes_path = env::var_os("SYNTAXES_PATH").unwrap();

    let options = grass::Options::default()
        .load_path(&picocss_path)
        .style(grass::OutputStyle::Compressed);
    let compiled_css =
        grass::from_path("scss/style.scss", &options).expect("Couldn't compile sass");
    let dest_file = Path::new(&out).join("style.css");
    fs::write(dest_file, compiled_css).expect("Couldn't write compiled css");

    let ss = syntect::parsing::SyntaxSet::load_from_folder(&syntaxes_path).unwrap();
    let dest_file = Path::new(&out).join("syntaxes.bin");
    fs::write(dest_file, syntect::dumps::dump_binary(&ss))
        .expect("Counldn't write compiled syntaxes");

    println!("cargo:rerun-if-changed=scss");
    println!(
        "cargo:rerun-if-changed=${}",
        syntaxes_path.to_str().unwrap()
    );
    println!("cargo:rerun-if-changed=${}", picocss_path.to_str().unwrap());
    println!("cargo:rerun-if-changed=build.rs");
}
