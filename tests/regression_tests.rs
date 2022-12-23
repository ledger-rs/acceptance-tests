/*
 * Ledger regression tests
 * This file mimics the logic of RegressTests.py script in Ledger repository.
 * It reuses the original Ledger .test files.
 */

use std::{fs, path::PathBuf};

use anyhow::Error;

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

        // todo: execute

        // todo: assert
    }

    todo!("complete");
}

#[derive(Debug, Default)]
struct Test {
    Command: String,
    Output: Vec<String>,
    Error: Option<String>,
    ExitCode: u16
}

fn read_test(contents: String) -> Test {
    let mut test = Test::default();
    let mut in_test = false;
    let mut in_error = false;

    for line in contents.lines() {
        if line.starts_with("test") {
            test.Command = line.to_string();
        }
        else if in_test {
            if line.starts_with("end test") {
                in_test = false;
                // todo: in_error = false;
            } else if in_error {
                if test.Error.is_none() {
                    //test.Error = []
                }
            } else {
                if test.Output.is_empty() {
                    test.Output.push(line.to_string());
                }
            }
        }
    }

    test
}