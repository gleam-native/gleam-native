// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

//! End-to-end tests for the native target: build a real project with the
//! `gleam` binary, JIT-run it, and check what it prints.

use std::path::PathBuf;
use std::process::Command;

/// A project directory in the system temp dir, removed on drop.
struct TestProject {
    root: PathBuf,
}

impl TestProject {
    fn new(name: &str, source: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "gleam-native-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src")).expect("create project directories");
        std::fs::write(
            root.join("gleam.toml"),
            format!("name = \"{name}\"\nversion = \"1.0.0\"\ntarget = \"native\"\n"),
        )
        .expect("write gleam.toml");
        std::fs::write(root.join("src").join(format!("{name}.gleam")), source)
            .expect("write source");
        Self { root }
    }

    fn run(&self) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gleam"))
            .arg("run")
            .current_dir(&self.root)
            .output()
            .expect("run the gleam binary")
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn fizzbuzz() {
    let project = TestProject::new(
        "fizzbuzz",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn fizzbuzz(n: Int, limit: Int) -> Nil {
  case n > limit {
    True -> Nil
    False -> {
      case n % 3, n % 5 {
        0, 0 -> println("FizzBuzz")
        0, _ -> println("Fizz")
        _, 0 -> println("Buzz")
        _, _ -> print_int(n)
      }
      fizzbuzz(n + 1, limit)
    }
  }
}

pub fn main() -> Nil {
  fizzbuzz(1, 15)
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n";
    assert!(
        stdout.contains(expected),
        "unexpected output.\nstdout: {stdout}"
    );
}
