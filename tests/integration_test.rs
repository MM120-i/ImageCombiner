use std::process::Command;
use std::path::Path;

#[test]
fn test_cli_help_flag() {
    let output: std::process::Output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--image-1"));
    assert!(stdout.contains("--image-2"));
    assert!(stdout.contains("-o, --output"));
}

#[test]
fn test_images_directory_exists() {
    let images_dir: &Path = Path::new("images");
    assert!(images_dir.exists());
    
    assert!(images_dir.join("image1.png").exists());
    assert!(images_dir.join("image2.png").exists());
}