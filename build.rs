use std::{fs, path::Path};

fn main() {
    let options = grass::Options::default().style(grass::OutputStyle::Compressed);
    let compiled_css =
        grass::from_path("scss/style.scss", &options).expect("Couldn't compile sass");

    let dest_file = Path::new("assets").join("style.css");
    fs::write(dest_file, compiled_css).expect("Couldn't write compiled css");

    println!("cargo:rerun-if-changed=scss");
    println!("cargo:rerun-if-changed=build.rs");
}
