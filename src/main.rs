use std::path::PathBuf;

use serde_yaml::Value;

pub mod backends;
pub mod color;
pub mod logger;
mod tests;

static HELP: &str = r#"Usage: chick [OPTIONS...] [TARGETS...]

    -h, --help      display this help
    -v, --verbose   enable verbose mode

Errors should be reported on Github Issues for this project:
https://github.com/falcolabs/chick/issues.
            This chick from falcolabs does not have Super Cow Ability.
"#;

#[derive(Debug, Clone)]
struct Task {
    pub name: String,
    pub commands: Vec<String>,
}

#[inline(always)]
fn buildstep_runquiet(task: Task, project_root: PathBuf, package_name: &str) {
    logger::info(format!("Executing build step &d{}&r", task.name));
    let cmd = task.commands.join(";");
    logger::debug(format!("{}", cmd));
    chick_panic(
        std::env::set_current_dir(project_root.clone()),
        "Cannot return to project root directory.",
    );
    let output = chick_panic(
        std::process::Command::new("sh").arg("-c").arg(cmd).output(),
        &format!("Unable to run build step &d{}&r", task.name),
    );
    if !output.status.success() {
        logger::error(format!("Failed to run build step &d{}&r", task.name));
        logger::error("Process stdout:");
        println!("{:#?}", String::from_utf8(output.stdout).unwrap());
        logger::error("Process stderr:");
        println!("{:#?}", String::from_utf8(output.stderr).unwrap());
        logger::error(format!(
            "Package &6{}&r build failed. See above error message for more information.",
            package_name
        ));
        std::process::exit(1);
    }
    logger::success(format!("Build step &d{}&r completed.", task.name));
}

#[inline(always)]
fn buildstep_runloud(task: Task, project_root: PathBuf, package_name: &str) {
    logger::info(format!("Executing build step &d{}&r", task.name));
    let cmd = task.commands.join(";");
    logger::debug(format!("{}", cmd));
    chick_panic(
        std::env::set_current_dir(project_root.clone()),
        "Cannot return to project root directory.",
    );
    let status = chick_panic(
        std::process::Command::new("sh").arg("-c").arg(cmd).status(),
        &format!("Unable to run build step &d{}&r", task.name),
    );
    if !status.success() {
        logger::error(format!("Failed to run build step &d{}&r", task.name));
        logger::error(format!(
            "Package &6{}&r build failed. See above build log for more information.",
            package_name
        ));
        std::process::exit(1);
    }
    logger::success(format!("Build step &d{}&r completed.", task.name));
}

fn chick_panic<T, E: std::fmt::Display + std::fmt::Debug>(result: Result<T, E>, error: &str) -> T {
    match result {
        Err(e) => {
            // TODO - optional error message switch
            logger::error(error);
            logger::error(e.to_string());
            std::process::exit(1);
        }
        Ok(t) => return t,
    }
}

fn chick_unwrap<T>(option: Option<T>, error: &str) -> T {
    match option {
        None => {
            logger::error(error);
            std::process::exit(1);
        }
        Some(t) => return t,
    }
}

fn main() {
    let mut is_verbose = false;

    let args: Vec<String> = {
        let mut output: Vec<String> = Vec::new();
        for a in std::env::args() {
            if a.starts_with("--") || a.starts_with("-") {
                match a.as_str() {
                    "--verbose" | "-v" => {
                        logger::set_log_level(logger::Level::DEBUG);
                        is_verbose = true;
                    }
                    "--help" | "-h" => {
                        print!("{}", HELP);
                        std::process::exit(0);
                    }
                    _ => {
                        logger::error(format!(
                            "Unrecognized switch &c{}&r. Try using &a--help&r for valid switches.",
                            a
                        ));
                        std::process::exit(0);
                    }
                }
            } else {
                output.push(a);
            }
        }
        output
    };

    let configuration: Value = chick_panic(
        serde_yaml::from_str(
            chick_panic(
                std::fs::read_to_string("chick.yaml"),
                "Failed to read chick.yaml",
            )
            .as_str(),
        ),
        "Failed to deserialize chick.yaml",
    );
    let package_name = chick_unwrap(
        configuration.get("name"),
        "Property 'name' is not defined in chick.yaml",
    )
    .as_str()
    .unwrap();
    if !is_verbose {
        logger::info(format!(
            "Building project &6{}&r &l&7(output hidden)&r",
            package_name
        ));
    } else {
        logger::info(format!("Building project &6{}&r", package_name));
    }
    let build_root = std::env::current_dir().unwrap();
    let mut targets: Vec<String> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();
    if args.len() == 1 {
        targets.push("default".to_string());
    } else {
        targets = args[1..].to_vec();
    }

    for t in targets {
        tasks.push(Task {
            name: t.clone(),
            commands: chick_unwrap(
                chick_unwrap(
                    configuration.get(t.clone()),
                    &format!("Target '{}' not found.", t),
                )
                .as_sequence(),
                &format!("Invalid target definition for '{}'", t),
            )
            .into_iter()
            .map(|e| {
                chick_unwrap(
                    e.as_str(),
                    &format!("Invalid syntax in target '{}' definition", t),
                )
                .to_string()
            })
            .collect(),
        })
    }

    if !is_verbose {
        for t in tasks {
            buildstep_runquiet(t, build_root.clone(), package_name);
        }
    } else {
        for t in tasks {
            buildstep_runloud(t, build_root.clone(), package_name);
        }
    }
    logger::success(format!(
        "Package &6{}&r built successfully. No errors reported.",
        package_name
    ));
}
