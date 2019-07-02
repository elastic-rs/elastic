use std::{
    error::Error,
    path::{
        Path,
        PathBuf,
    },
    process::{
        Command,
        Stdio,
    },
};

struct Names {
    container_name: String,
    build_name: String,
    dockerfile_name: String,
}

impl Names {
    fn from_run(run: &str) -> Self {
        let container_name = format!("elastic-rs-{}", run);
        let build_name = format!("{}:latest", container_name);
        let dockerfile_name = format!("{}.Dockerfile", run);

        Names {
            container_name: container_name,
            build_name: build_name,
            dockerfile_name: dockerfile_name,
        }
    }
}

pub fn start(run: &str) -> Result<(), Box<dyn Error>> {
    let names = Names::from_run(run);
    let containers_path = "tests/integration/containers";

    let mut dockerfile_path = PathBuf::from(containers_path);
    dockerfile_path.push(&names.dockerfile_name);

    if Path::exists(dockerfile_path.as_ref()) {
        // Kill the container if it's runnning
        kill(run)?;

        // Build the container
        Command::new("docker")
            .args(&[
                "build",
                "-f",
                &names.dockerfile_name,
                "-t",
                &names.build_name,
                ".",
            ])
            .current_dir(containers_path)
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
        println!("'{:?}' not found, skipping docker setup", dockerfile_path);

        Ok(())
    }
}

pub fn kill(run: &str) -> Result<(), Box<dyn Error>> {
    let names = Names::from_run(run);

    // Kill the container if it's runnning
    Command::new("docker")
        .args(&["rm", "-f", &names.container_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}
