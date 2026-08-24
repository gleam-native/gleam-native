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
fn tuples() {
    let project = TestProject::new(
        "tuples",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_float")
pub fn print_float(value: Float) -> Nil

fn swap(pair: #(Int, Int)) -> #(Int, Int) {
  case pair {
    #(first, second) -> #(second, first)
  }
}

fn scale(point: #(Float, Float), factor: Float) -> #(Float, Float) {
  #(point.0 *. factor, point.1 *. factor)
}

fn describe(pair: #(Int, String)) -> String {
  case pair {
    #(1, word) -> "one " <> word
    #(_, word) if pair.0 > 100 -> "many " <> word
    #(_, word) -> "some " <> word
  }
}

pub fn main() -> Nil {
  // Access, including nested tuples.
  let nested = #(1, #(2, 3))
  print_int(nested.0)
  print_int(nested.1.0 + nested.1.1)

  // Destructuring and reconstruction.
  let swapped = swap(#(40, 2))
  print_int(swapped.0 - swapped.1)

  // Mixed element types and tuple index in guards.
  println(describe(#(1, "apple")))
  println(describe(#(999, "grapes")))
  println(describe(#(7, "pears")))

  // Tuples of floats through a function.
  let scaled = scale(#(1.5, 2.5), 2.0)
  print_float(scaled.0)
  print_float(scaled.1)
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
    let expected = "1\n5\n-38\none apple\nmany grapes\nsome pears\n3.0\n5.0\n";
    assert!(
        stdout.contains(expected),
        "unexpected output.\nstdout: {stdout}"
    );
}

#[test]
fn lists() {
    let project = TestProject::new(
        "lists",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn sum(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [first, ..rest] -> first + sum(rest)
  }
}

fn length(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [_, ..rest] -> 1 + length(rest)
  }
}

fn reverse_into(list: List(Int), accumulator: List(Int)) -> List(Int) {
  case list {
    [] -> accumulator
    [first, ..rest] -> reverse_into(rest, [first, ..accumulator])
  }
}

fn join(words: List(String)) -> String {
  case words {
    [] -> ""
    [word] -> word
    [word, ..rest] -> word <> " " <> join(rest)
  }
}

fn shape(list: List(Int)) -> String {
  case list {
    [] -> "empty"
    [_] -> "single"
    [first, second] if first == second -> "twin pair"
    [_, _] -> "pair"
    _ -> "longer"
  }
}

pub fn main() -> Nil {
  let numbers = [5, 1, 9, 3]
  print_int(sum(numbers))
  print_int(length(numbers))
  print_int(sum([40, ..[2]]))

  // Reversal: sum of leading pair differs after reversing.
  case reverse_into(numbers, []) {
    [first, second, ..] -> print_int(first * 10 + second)
    _ -> print_int(-1)
  }

  println(join(["lists", "work", "natively"]))

  println(shape([]))
  println(shape([1]))
  println(shape([4, 4]))
  println(shape([4, 5]))
  println(shape(numbers))
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
    let expected =
        "18\n4\n42\n39\nlists work natively\nempty\nsingle\ntwin pair\npair\nlonger\n";
    assert!(
        stdout.contains(expected),
        "unexpected output.\nstdout: {stdout}"
    );
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
