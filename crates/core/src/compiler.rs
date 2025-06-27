use std::path::Path;
use std::process::Command;

pub trait Compiler {
    fn compile(
        &self,
        source_path: &Path,
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
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
        source_path: &Path,
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("g++")
            .arg(source_path)
            .arg("-std=c++20")
            .arg("-O2")
            .arg("-o")
            .arg(output_path)
            .output()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Box::from(format!("Compilation failed: {}", stderr)));
        }

        Ok(())
    }
}

impl Default for CppCompiler {
    fn default() -> Self {
        Self::new()
    }
}
