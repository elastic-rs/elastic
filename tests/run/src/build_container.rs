use std::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};

struct Names {
    container_name: String,
    build_name: String,
    build_path: String,
}

impl Names {
    fn from_run(run: &str) -> Self {
        let container_name = format!("elastic-rs-{}", run);
        let build_name = format!("{}:latest", container_name);
        let build_path = format!("./containers/{}.Dockerfile", run);

        Names {
            container_name: container_name,
            build_name: build_name,
            build_path: build_path,
        }
    }
}

pub fn start(run: &str) -> Result<(), Box<Error>> {
    let names = Names::from_run(run);

    if Path::exists(names.build_path.as_ref()) {
        // Kill the container if it's runnning
        kill(run)?;

        // Build the container
        Command::new("docker")
            .args(&[
                "build",
                "-f",
                &names.build_path,
                "-t",
                &names.build_name,
                ".",
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        // Start the container
        Command::new("docker")
            .args(&[
                "run",
                "-itd",
                "-p",
                "9200:9200",
                "--name",
                &names.container_name,
                &names.build_name,
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;

        Ok(())
    } else {
        println!("'{}' not found, skipping docker setup", names.build_path);

        Ok(())
    }
}

pub fn kill(run: &str) -> Result<(), Box<Error>> {
    let names = Names::from_run(run);

    // Kill the container if it's runnning
    Command::new("docker")
        .args(&["rm", "-f", &names.container_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
