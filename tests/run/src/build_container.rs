use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn call(run: &str) -> Result<(), Box<Error>> {
    let name = format!("elastic-rs-{}", run);
    let image_name = format!("{}:latest", name);
    let image_file = format!("./containers/{}.Dockerfile", run);

    if Path::exists(image_file.as_ref()) {
        // Kill the container if it's runnning
        Command::new("docker")
            .args(&["rm", "-f", &name])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        // Build the container
        Command::new("docker")
            .args(&["build", "-f", &image_file, "-t", &image_name, "."])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        // Start the container
        Command::new("docker")
            .args(&["run", "-itd", "-p", "9200:9200", "--name", &name, &image_name])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        Ok(())
    } else {
        println!("'{}' not found, skipping docker setup", image_file);

        Ok(())
    }
}
