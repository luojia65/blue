#[cfg(windows)]
fn main() {    
    println!("cargo:rustc-link-lib=Bthprops");
}

#[cfg(unix)]
fn main() {    
}

#[cfg(not(any(windows, unix)))]
fn main() {
    panic!("target not supported. by now blue only support windows")
}
