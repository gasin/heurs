use std::path::PathBuf;
use std::process::Command;

pub trait Compiler {
    fn compile(
        &self,
        source_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct CppCompiler;

impl CppCompiler {
    pub fn new() -> Self {
        CppCompiler
    }
}

impl Compiler for CppCompiler {
    fn compile(
        &self,
        source_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("g++")
            .arg(source_path)
            .arg("-std=c++20")
            .arg("-O2")
            .arg("-o")
            .arg(output_path)
            .output()?;

        if !output.status.success() {
            return Err(format!(
                "Compilation failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(())
    }
}

impl Default for CppCompiler {
    fn default() -> Self {
        Self::new()
    }
}
