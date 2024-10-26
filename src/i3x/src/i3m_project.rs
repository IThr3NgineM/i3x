use std::process::Command;
use std::fs::{self, File};
use anyhow::Result;

pub fn create(name: String) -> Result<()> {
    let dir = format!("./{}", name);

    if std::path::Path::new(&dir).exists() {
        return Err(anyhow::anyhow!("Project already exists"));
    }

    fs::create_dir_all(&dir)?;
    File::create(format!("{}/Cargo.toml", dir))?;
    File::create(format!("{}/main.rs", dir))?;

    println!("New I3M project '{}' created.", name);
    Ok(())
}

pub fn build(proj: String) -> Result<()> {
    let output = Command::new("cargo")
        .arg("build")
        .current_dir(proj)
        .output()
        .expect("Failed to build project");

    if !output.status.success() {
        return Err(anyhow::anyhow!("Build failed"));
    }

    println!("Project built successfully.");
    Ok(())
}

pub fn run(proj: String) -> Result<()> {
    let output = Command::new("cargo")
        .arg("run")
        .current_dir(proj)
        .output()
        .expect("Failed to run project");

    if !output.status.success() {
        return Err(anyhow::anyhow!("Run failed"));
    }

    println!("Project running.");
    Ok(())
}

pub fn initialize_core() -> Result<()> {
    println!("Initializing I3M Engine Core...");
    // Initialize core engine components
    Ok(())
}
