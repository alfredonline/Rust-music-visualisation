use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> std::io::Result<()> {
    println!("Script starting...");
    
    // Build release version first
    println!("Building release version...");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .status()?;
    
    if !status.success() {
        println!("Failed to build release version!");
        return Ok(());
    }

    // Create release directory
    let release_dir = Path::new("release");
    fs::create_dir_all(release_dir)?;

    // Verify executable exists
    let exe_path = Path::new("target/release/platformer.exe");
    if !exe_path.exists() {
        println!("Error: Could not find executable at {:?}", exe_path);
        return Ok(());
    }

    // Copy the executable
    println!("Copying executable from {:?}", exe_path);
    fs::copy(exe_path, release_dir.join("platformer.exe"))?;

    // Copy SDL2 DLLs from debug folder
    let sdl2_dlls = [
        "SDL2.dll",
        "SDL2_ttf.dll",
        "libfreetype-6.dll",
        "zlib1.dll",
        "freetype.dll",
        "brotlidec.dll",
        "libpng16.dll",
        "bz2.dll",
        "brotlicommon.dll",
    ];

    println!("Copying DLLs...");
    for dll in sdl2_dlls.iter() {
        let source = format!("target/debug/{}", dll);
        let dest = release_dir.join(dll);
        println!("Copying {} to {}", source, dest.display());
        match fs::copy(&source, &dest) {
            Ok(_) => println!("Successfully copied {}", dll),
            Err(e) => println!("Failed to copy {}: {}", dll, e),
        }
    }

    // Create assets directory structure
    println!("Creating assets directory...");
    fs::create_dir_all(release_dir.join("assets/fonts"))?;

    // Copy font assets
    println!("Copying font assets...");
    let font_paths = [
        "src/assets/fonts/times.ttf",
        "assets/fonts/times.ttf",
        "times.ttf"
    ];

    let mut font_copied = false;
    for font_path in font_paths.iter() {
        if Path::new(font_path).exists() {
            match fs::copy(font_path, release_dir.join("assets/fonts/times.ttf")) {
                Ok(_) => {
                    println!("Successfully copied font from {}", font_path);
                    font_copied = true;
                    break;
                },
                Err(e) => println!("Failed to copy font from {}: {}", font_path, e),
            }
        } else {
            println!("Font path not found: {}", font_path);
        }
    }

    if !font_copied {
        println!("WARNING: Could not copy font file from any location!");
        println!("Please ensure times.ttf exists in one of these locations:");
        for path in font_paths.iter() {
            println!("  - {}", path);
        }
    }

    println!("\nVerifying files in release directory:");
    for entry in fs::read_dir(release_dir)? {
        let entry = entry?;
        println!("Found: {}", entry.path().display());
    }
    
    Ok(())
} 