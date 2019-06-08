use std::{error::Error, fmt, fs::read_dir, fs::File, io::Read, path::Path, process::Command};

fn main() {
    _main().unwrap();
}

fn _main() -> Result<(), Box<Error>> {
    let mut paths: Vec<_> = read_dir("./examples_working")?
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|d| d.path());
    for entry in paths {
        let path = entry.path();
        if "ds" == path.extension().unwrap() {
            let output = execute_script(path.as_path())?;
            let expected_output = get_expected_output(path.as_path())?;
            if output == expected_output {
                println!("{:?}: passed", entry.path());
            } else {
                println!("{:?}: failed", entry.path());
                println!("expected output:\n{}", expected_output);
                println!("actual output:\n{}", output);
            }
        }
    }
    Ok(())
}

/// execute a script, and return the output as a string.
fn execute_script(script_path: &Path) -> Result<String, Box<Error>> {
    let output = Command::new("./bin/disp").arg(script_path).output()?;
    match String::from_utf8(output.stdout) {
        Ok(s) => Ok(s),
        Err(err) => Err(Box::new(FullstackError::new(format!(
            "{:?}",
            err.as_bytes()
        )))),
    }
}

/// output is expected to be a file in the form out <script_name>.output
/// alongside the script of the same name.
fn get_expected_output(script_path: &Path) -> Result<String, Box<Error>> {
    let path_with_output_extension = script_path.to_owned().with_extension("output");
    let mut output_file = File::open(path_with_output_extension)?;
    let mut input = String::new();
    output_file.read_to_string(&mut input)?;
    Ok(input)
}

#[derive(Debug)]
struct FullstackError {
    message: String,
}

impl FullstackError {
    pub fn new(message: String) -> FullstackError {
        FullstackError { message }
    }
}

impl fmt::Display for FullstackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for FullstackError {}
