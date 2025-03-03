fn main() {
    println!("cargo:rustc-link-search=native=C:/Users/afort/vcpkg/installed/x64-windows/lib");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_ttf");  // Add this line for TTF support
}