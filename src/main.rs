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
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("--verbose")) || args.contains(&String::from("-v")) {
        logger::set_log_level(logger::Level::DEBUG);
    }

    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print!("{}", HELP);
        std::process::exit(0);
    }
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
    logger::info(format!("Building project &6{}&r", package_name));
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

    for t in tasks {
        logger::info(format!("Executing build step &d{}&r", t.name));
        chick_panic(
            std::env::set_current_dir(build_root.clone()),
            "Cannot return to project root directory.",
        );
        for c in t.commands {
            let output = chick_panic(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(c.clone())
                    .output(),
                &format!("Unable to run command '{}'", c),
            );
            if !output.status.success() {
                logger::error(format!("Failed to run build step '{}'", t.name));
                logger::error(format!(
                    "Detailed build log: {}",
                    String::from_utf8(output.stdout).unwrap()
                ));
                logger::success(format!(
                    "Package &6{}&r build failed. See above error message for more information.",
                    package_name
                ));
                std::process::exit(1);
            }
        }
        logger::success(format!("Build step &d{}&r completed.", t.name));
    }
    logger::success(format!(
        "Package &6{}&r built successfully. No errors reported.",
        package_name
    ));
}
