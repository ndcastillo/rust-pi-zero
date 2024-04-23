use std::process::Command;
use std::io::{self, Write};

fn handler(bash_script: &str) -> Result<String, String> {
    println!("Detectando Puertos USB ...");

    let output = Command::new("sh")
        .arg("-c")
        .arg(bash_script)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                eprintln!("Error ejecutando el script: {}", stderr);
                Err(stderr)
            }
        },
        Err(error) => {
            eprintln!("Error al ejecutar comando: {}", error);
            Err(error.to_string())
        }
    }
}

fn main() {
    match handler("bash.sh") {
        Ok(result) => println!("Salida del script: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
