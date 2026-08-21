use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("discord-codegen: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage());
    };
    if command != "generate" {
        return Err(usage());
    }

    let input = option_value(&args, "--input")?.ok_or_else(usage)?;
    let output = option_value(&args, "--output")?;
    let input_path = PathBuf::from(input);
    let document = fs::read_to_string(&input_path)
        .map_err(|error| format!("cannot read {}: {error}", input_path.display()))?;
    let generated = discord_codegen::generate(&document).map_err(|error| error.to_string())?;

    if let Some(output) = output {
        let output_path = PathBuf::from(output);
        fs::write(&output_path, generated)
            .map_err(|error| format!("cannot write {}: {error}", output_path.display()))?;
    } else {
        print!("{generated}");
    }

    Ok(())
}

fn option_value(args: &[String], option: &str) -> Result<Option<String>, String> {
    let Some(index) = args.iter().position(|argument| argument == option) else {
        return Ok(None);
    };
    args.get(index + 1)
        .cloned()
        .map(Some)
        .ok_or_else(|| format!("missing value for {option}\n\n{}", usage()))
}

fn usage() -> String {
    "usage: discord-codegen generate --input <openapi.json> [--output <generated.rs>]".to_owned()
}
