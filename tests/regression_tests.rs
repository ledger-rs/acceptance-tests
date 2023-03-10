/*
 * Ledger regression tests
 * This file mimics the logic of RegressTests.py script in Ledger repository.
 * It reuses the original Ledger .test files.
 */

use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::Command,
};

use anyhow::{Error, Ok};
use regex::{Regex, RegexBuilder};

#[test]
fn regression_tests() -> Result<(), Error> {
    // read *.test files in all subdirectories.
    // "unit" - only C++ files
    // "manual"
    // "baseline"
    // "regress"

    for entry in glob::glob("**/*.test")? {
        let path = entry.expect("test file");

        // parse
        let contents = fs::read_to_string(&path)?;
        //let file = fs::File::open(&path)?;

        // while let test = read_test(&file)? {
        let tests = read_tests_via_regex(&contents);
        println!("parsed tests: {:?}", tests);

        for test in tests {
            // execute
            let filename = path.to_str().unwrap().to_string();
            let result = run_test(test, &filename)?;

            // todo: assert
            // let expected = result.Output;
            let actual: Vec<String> = vec![];
            // assert_eq!(expected, actual);
        }
    }

    assert!(false);

    Ok(())
}

#[derive(Debug, Default)]
struct Test {
    Command: String,
    Output: Vec<String>,
    Error: Vec<String>,
    ExitCode: u16,
}

fn read_test(file: &File) -> Result<Test, Error> {
    // contents: String

    let mut test = Test::default();
    let mut in_output = false;
    let mut in_error = false;

    let reader = BufReader::new(file);
    //for line in contents.lines() {
    for line in reader.lines() {
        let line = line?;

        if line.starts_with("test") {
            let command = line[5..].to_string();

            let match_regex = Regex::new(r"(.*) -> ([0-9]+)").unwrap();
            //let matches = match_regex.is_match(&command);
            if let Some(captures) = match_regex.captures(&command) {
                // todo: transform line
                test.Command = captures.get(1).unwrap().as_str().to_owned();
                test.ExitCode = captures.get(2).unwrap().as_str().parse().unwrap();
            } else {
                test.Command = command;
            }
            in_output = true;
        } else if in_output {
            if line.starts_with("end test") {
                in_output = false;
                in_error = false;
                break;
            } else if in_error {
                // todo: transform line
                test.Error.push(line.to_string());
            } else {
                if line.starts_with("__ERROR__") {
                    in_error = true;
                } else {
                    // todo: transform line
                    test.Output.push(line.to_string());
                }
            }
        }
    }

    Ok(test)
}

fn run_test(mut test: Test, filename: &String) -> Result<Test, Error> {
    let mut use_stdin = false;

    if cfg!(windows) {
        test.Command = test.Command.replace("/dev/null", "nul");

        if test.Command.contains("/dev/std") {
            todo!("mark success");
            //return;
        }
    }

    if test.Command.contains("-f") {
        test.Command = "ledger ".to_string() + test.Command.as_str();

        let re = Regex::new(r#"-f (-|/dev/stdin)(\s|$)"#).unwrap();
        if re.is_match(&test.Command) {
            use_stdin = true;
        }
    } else {
        test.Command = format!(r#"ledger -f "{}" {}"#, filename, test.Command);
    }

    // execute test
    let cmd = Command::new(&test.Command).spawn()?;

    if use_stdin {
        // read from file into stdin.
        let input = fs::read_to_string(filename)?;
        //cmd.stdin.
    }

    if !test.Output.is_empty() {
        let mut output = String::default();
        let _read = cmd.stdout.unwrap().read_to_string(&mut output)?;

        if cfg!(windows) {
            // target_os = "windows") {
            todo!("complete");
            // output = for line in output.lines() {
            //     line.replace("\r\n", "\n")
            // }
        }
    }

    // Ok(output.to_owned())
    Ok(test)
}

fn read_tests_via_regex(contents: &String) -> Vec<Test> {
    // get everything between "test " and "end test", on separate lines.
    let pattern = r"^test ([\s\S]+?)^end test$";

    let re = RegexBuilder::new(pattern).multi_line(true).build().unwrap();

    re.captures_iter(contents)
        .map(|captures| captures.get(1).unwrap().as_str())
        .map(|test_string| parse_test(test_string))
        .collect()
}

fn parse_test(test_string: &str) -> Test {
    let mut test = Test::default();

    let lines: Vec<String> = test_string.lines().map(|line| line.to_owned()).collect();
    test.Command = lines[0].to_owned();
    test.Output = lines[1..].to_vec();

    test
}
