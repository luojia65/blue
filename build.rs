use std::env::var;

fn main() {    
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) != Some(&"windows") {
        panic!("Target not supported")
    }
    println!("cargo:rustc-link-lib=Bthprops");
}
