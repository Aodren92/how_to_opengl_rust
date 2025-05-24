fn main() {
    println!("cargo::rustc-link-lib=SDL3");
    println!("cargo::rustc-link-search=./sdl/build/SDL");
}
