#[cfg(windows)]
fn main() {    
    println!("cargo:rustc-link-lib=Bthprops");
}

#[cfg(not(windows))]
fn main() {
    panic!("target not supported. by now blue only support windows")
}
