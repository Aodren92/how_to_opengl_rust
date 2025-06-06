fn main() {
    println!("cargo::rustc-link-search=./libs");
    println!("cargo::rustc-link-lib=SDL3");
    println!("cargo::rustc-link-lib=SDL3_image");
   // println!("cargo::rustc-link-search=./SDL_image");
}
