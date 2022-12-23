/*
 * Ledger regression tests
 * This file mimics the logic of RegressTests.py script in Ledger repository.
 * It reuses the original Ledger .test files.
 */

use std::{fs, io::Read, path::PathBuf, process::Command};

use anyhow::{Error, Ok};
use regex::Regex;

#[test]
fn regression_tests() -> Result<(), Error> {
    // read *.test files in all subdirectories.
    // "unit" - only C++ files
    // "manual"
    // "baseline"
    // "regress"

    for entry in glob::glob("**/*.test")? {
        let path = entry.expect("test file");

        // todo: parse
        let contents = fs::read_to_string(&path)?;

        let test = read_test(contents);

        println!("parsed test: {:?}", test);

        // execute
        let result = run_test(test, path.to_str().unwrap().to_string())?;

        // todo: assert
        let expected = result.Output;
        let actual: Vec<String> = vec![];
        assert_eq!(expected, actual);
    }

    Ok(())
}

#[derive(Debug, Default)]
struct Test {
    Command: String,
    Output: Vec<String>,
    Error: Vec<String>,
    ExitCode: u16,
}

fn read_test(contents: String) -> Test {
    let mut test = Test::default();
    let mut in_output = false;
    let mut in_error = false;

    for line in contents.lines() {
        if line.starts_with("test") {
            let command = line[5..].to_string();

            let match_regex = Regex::new(r"(.*) -> ([0-9]+)").unwrap();
            //let matches = match_regex.is_match(&command);
            if let Some(captures) = match_regex.captures(&command) {
                todo!("complete");
                //todo: test.Command =
                //todo: test.ExitCode = match_regex.captures(line);
                //captures.get(2)
            } else {
                test.Command = command;
            }
        } else if in_output {
            if line.starts_with("end test") {
                in_output = false;
                // todo: in_error = false;
                todo!("complete");
                break;
            } else if in_error {
                test.Error.push(line.to_string());
            } else {
                if line.starts_with("__ERROR__") {
                    in_error = true;
                } else {
                    test.Output.push(line.to_string());
                }
            }
        }
    }

    test
}

fn run_test(mut test: Test, filename: String) -> Result<Test, Error> {
    if cfg!(target_os = "windows") {
        test.Command = test.Command.replace("/dev/null", "nul");

        if test.Command.contains("/dev/std") {
            todo!("mark success");
        }
    }

    if test.Command.contains("-f") {
        test.Command = "ledger ".to_string() + test.Command.as_str();

        todo!("complete");
    } else {
        test.Command = format!(r#"ledger -f "{}" {}"#, filename, test.Command);
    }

    // todo: if use stdin

    if !test.Output.is_empty() {
        let cmd = Command::new(&test.Command).spawn()?;
        let mut output = String::default();
        let _read = cmd.stdout.unwrap().read_to_string(&mut output)?;

        if cfg!(target_os = "windows") {
            todo!("complete");
            // output = for line in output.lines() {
            //     line.replace("\r\n", "\n")
            // }
        }
    }

    // Ok(output.to_owned())
    Ok(test)
}
