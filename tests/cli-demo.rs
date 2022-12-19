/*
 * Demonstrate the recommended best practices for testing CLI applications.
 *
 * References:
 * - https://rust-cli.github.io/book/tutorial/testing.html
 */

use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

/// Example from the CLI book.
/// This is utilizing external crates to test the output of the project's binary.
#[test]
fn first_test() -> Result<(), Box<dyn std::error::Error>> {
    // Command::spawn(&mut self)
    Command::cargo_bin("ledger-cli-rs")?
        .assert()
        //.failure()
        //.stderr(predicate::str::contains("Some error"));
        .success()
        .stdout(predicate::str::contains("Hello, plain-text accountants!\n"));

    Ok(())
}

/// Executing any CLI tool and collecting the output.
#[test]
fn with_new() {
    let mut cmd = Command::new("ledger");
    //cmd.args(args)
    let output = cmd.output().expect("this should run the cli");
    let result = String::from_utf8(output.stdout).expect("provide stdout text");

    assert_ne!(result, String::default());
    assert_eq!(result, "Ledger 3.2.1-20200518, the command-line accounting tool\r\n\r\nCopyright (c) 2003-2019, John Wiegley.  All rights reserved.\r\n\r\nThis program is made available under the terms of the BSD Public License.\r\nSee LICENSE file included with the distribution for details and disclaimer.\r\n");
}
