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
        Self::with_config_extras(name, source, "")
    }

    fn with_config_extras(name: &str, source: &str, config_extras: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "gleam-native-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src")).expect("create project directories");
        std::fs::write(
            root.join("gleam.toml"),
            format!(
                "name = \"{name}\"\nversion = \"1.0.0\"\ntarget = \"native\"\n{config_extras}"
            ),
        )
        .expect("write gleam.toml");
        std::fs::write(root.join("src").join(format!("{name}.gleam")), source)
            .expect("write source");
        Self { root }
    }

    fn run(&self) -> std::process::Output {
        self.run_with_arguments(&[])
    }

    fn run_with_arguments(&self, arguments: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gleam"))
            .arg("run")
            .args(arguments)
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
fn bit_arrays() {
    let project = TestProject::new(
        "bit_arrays",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_float")
pub fn print_float(value: Float) -> Nil

@external(native, "runtime", "print_bool")
pub fn print_bool(value: Bool) -> Nil

fn parse_frame(packet: BitArray) -> String {
  // Length-prefixed frame: 1-byte length, then that many bytes.
  case packet {
    <<length, payload:bytes-size(length), rest:bits>> ->
      case payload, rest {
        <<"hi">>, <<>> -> "greeting"
        _, _ ->
          case length {
            3 -> "frame of three"
            _ -> "other frame"
          }
      }
    _ -> "malformed"
  }
}

pub fn main() -> Nil {
  // Dynamic length-prefixed parsing, including rejection.
  println(parse_frame(<<2, "hi":utf8>>))
  println(parse_frame(<<3, 7, 8, 9>>))
  println(parse_frame(<<5, 1>>))

  // Unaligned segments and the Erlang reference value for <<1000:12>>.
  let assert <<first, tail:4>> = <<1000:12>>
  print_int(first)
  print_int(tail)
  case <<1:1, 0:1, 1:1, 0:5, 10:4, 3:4>> {
    <<a:1, _:1, c:1, _:5, high:4, low:4>> -> {
      print_int(a * 100 + c * 10 + high - low)
    }
    _ -> println("no")
  }

  // Endianness and signedness.
  let assert <<value:16-little>> = <<1, 2>>
  print_int(value)
  case <<255>> {
    <<n:signed>> -> print_int(n)
    _ -> println("no")
  }

  // Floats at 16, 32, and 64 bits round-trip through patterns.
  case <<1.5:16, 2.5:32-little, 3.25>> {
    <<a:16-float, b:32-float-little, c:float>> -> {
      print_float(a)
      print_float(b)
      print_float(c)
    }
    _ -> println("no")
  }

  // Wide reads produce big integers; sizes may use arithmetic.
  let width = 2
  let assert <<head:size(width * 8), _:bits>> = <<513:16, 42>>
  print_int(head)
  let assert <<wide:size(128)>> = <<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0>>
  print_bool(wide == 256)

  // Encodings, native endianness, splices, and equality.
  print_bool(<<"AB":utf16-little>> == <<65, 0, 66, 0>>)
  print_bool(<<258:16-native>> == <<258:16-little>>)
  print_bool(<<<<0xAB>>:bits-size(4)>> == <<0xA:4>>)
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
    let expected = "greeting\nframe of three\nmalformed\n62\n8\n117\n513\n-1\n1.5\n2.5\n3.25\n513\nTrue\nTrue\nTrue\nTrue\n";
    assert!(
        stdout.contains(expected),
        "unexpected output.\nstdout: {stdout}"
    );
}

#[test]
fn custom_types() {
    let project = TestProject::new(
        "custom_types",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_float")
pub fn print_float(value: Float) -> Nil

@external(native, "runtime", "print_bool")
pub fn print_bool(value: Bool) -> Nil

pub type Shape {
  Circle(radius: Float)
  Rect(width: Float, height: Float)
  Point
}

pub type Person {
  Person(name: String, age: Int)
}

fn area(shape: Shape) -> Float {
  case shape {
    Circle(radius) -> 3.0 *. radius *. radius
    Rect(width, height) -> width *. height
    Point -> 0.0
  }
}

fn birthday(person: Person) -> Person {
  Person(..person, age: person.age + 1)
}

fn safe_div(a: Int, b: Int) -> Result(Int, String) {
  case b == 0 {
    True -> Error("division by zero")
    False -> Ok(a / b)
  }
}

fn show(result: Result(Int, String)) -> Nil {
  case result {
    Ok(value) -> print_int(value)
    Error(message) -> println(message)
  }
}

pub fn main() -> Nil {
  print_float(area(Circle(2.0)))
  print_float(area(Rect(1.5, 2.0)))
  print_float(area(Point))
  let alice = Person("Alice", 30)
  let older = birthday(alice)
  println(older.name)
  print_int(older.age)
  // Records are immutable: the original is untouched.
  print_int(alice.age)
  show(safe_div(84, 2))
  show(safe_div(1, 0))
  // Deep structural equality.
  print_bool(older == Person("Alice", 31))
  print_bool(alice == older)
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "12.0
3.0
0.0
Alice
31
30
42
division by zero
True
False
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn closures_and_pipes() {
    let project = TestProject::new(
        "closures",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

pub type Box {
  Box(content: Int)
}

fn map(list: List(a), with: fn(a) -> b) -> List(b) {
  case list {
    [] -> []
    [first, ..rest] -> [with(first), ..map(rest, with)]
  }
}

fn fold(list: List(a), from: b, with: fn(b, a) -> b) -> b {
  case list {
    [] -> from
    [first, ..rest] -> fold(rest, with(from, first), with)
  }
}

fn double(x: Int) -> Int {
  x * 2
}

fn add(a: Int, b: Int) -> Int {
  a + b
}

fn make_adder(amount: Int) -> fn(Int) -> Int {
  fn(x) { x + amount }
}

fn twice(f: fn(Int) -> Int, x: Int) -> Int {
  f(f(x))
}

fn with_label(label: String, callback: fn() -> Nil) -> Nil {
  println(label)
  callback()
}

pub fn main() -> Nil {
  // A closure capturing a local, returned from a function.
  let add_ten = make_adder(10)
  print_int(add_ten(32))
  // Module functions as values through generic higher-order functions.
  let numbers = [1, 2, 3, 4]
  print_int(fold(map(numbers, double), 0, add))
  // A capturing lambda inside map.
  let offset = 100
  print_int(fold(map(numbers, fn(n) { n + offset }), 0, add))
  // Function captures and pipes.
  let add_five = add(5, _)
  print_int(numbers |> fold(0, add) |> add_five)
  // Passing a closure twice.
  print_int(twice(add_ten, 1))
  // A constructor as a function value.
  case map([7], Box) {
    [Box(content)] -> print_int(content)
    _ -> println("no")
  }
  // use expressions desugar to callbacks.
  use <- with_label("computing")
  print_int(42)
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "42
20
410
15
21
7
computing
42
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn case_matching_and_guards() {
    let project = TestProject::new(
        "case_matching",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

fn size(n: Int) -> String {
  case n {
    1 | 2 | 3 -> "small"
    100 -> "big"
    n if n < 0 -> "negative"
    _ -> "medium"
  }
}

fn same(a: Int, b: Int) -> String {
  case a, b {
    x, y if x == y -> "same"
    x, y if !{ x < y } -> "descending"
    _, _ -> "ascending"
  }
}

fn describe(word: String) -> String {
  case word {
    "hi" -> "greeting"
    "say " <> words -> words
    _ -> "unknown"
  }
}

fn floaty(f: Float) -> String {
  case f {
    1.5 -> "one and a half"
    _ -> "other"
  }
}

fn big(n: Int) -> String {
  case n {
    9223372036854775808 -> "big literal"
    _ -> "not"
  }
}

pub fn main() -> Nil {
  println(size(2))
  println(size(100))
  println(size(-5))
  println(size(50))
  println(same(3, 3))
  println(same(9, 4))
  println(same(2, 8))
  println(describe("hi"))
  println(describe("say hello"))
  println(describe("zap"))
  println(floaty(1.5))
  println(floaty(2.5))
  println(big(9223372036854775808))
  println(big(5))
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "small
big
negative
medium
same
descending
ascending
greeting
hello
unknown
one and a half
other
big literal
not
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn constants_and_arithmetic() {
    let project = TestProject::new(
        "arith",
        r#"@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_float")
pub fn print_float(value: Float) -> Nil

@external(native, "runtime", "print_bool")
pub fn print_bool(value: Bool) -> Nil

const answer = 42

const limits = #(1, 100)

const defaults = [42, 7]

fn sum(list: List(Int)) -> Int {
  case list {
    [] -> 0
    [first, ..rest] -> first + sum(rest)
  }
}

pub fn main() -> Nil {
  print_int(answer)
  print_int(limits.1 - limits.0)
  print_int(sum(defaults))
  // Big integer promotion and demotion.
  print_int(4611686018427387903 * 4)
  print_int(9223372036854775808 - 9223372036854775807)
  // Truncating division, dividend-sign remainder, zero rules.
  print_int(-7 / 2)
  print_int(-7 % 2)
  print_int(1 / 0)
  print_float(0.1 +. 0.2)
  print_float(1.0 /. 0.0)
  print_bool(2 <= 2)
  print_bool(9223372036854775808 > 5)
  print_bool([1, 2] == [1, 2])
  print_bool(#(1, "a") == #(1, "b"))
  // echo reports on standard error and passes its value through.
  print_int(echo 21 * 2)
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {stderr}"
    );
    let expected = "42
99
49
18446744073709551612
1
-3
-1
0
0.30000000000000004
0.0
True
True
True
False
42
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
    assert!(
        stderr.contains("arith:") && stderr.contains("42"),
        "expected echo output on stderr.
stderr: {stderr}"
    );
}

#[test]
fn destructuring_lets() {
    let project = TestProject::new(
        "destructuring",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

pub type Config {
  Config(host: String, port: Int)
}

fn found(value: Int) -> Result(Int, String) {
  Ok(value)
}

pub fn main() -> Nil {
  let #(a, b) = #(40, 2)
  print_int(a + b)
  let Config(host: host, port: port) = Config("localhost", 8080)
  println(host)
  print_int(port)
  let assert [first, ..rest] = [1, 2, 3]
  let assert [second, ..] = rest
  print_int(first + second)
  let assert Ok(value) = found(9000)
  print_int(value)
  let assert "gleam-" <> version = "gleam-1.18"
  println(version)
  let assert <<x:16>> = <<2, 1>>
  print_int(x)
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "42
localhost
8080
3
9000
1.18
513
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn runtime_failures() {
    // Each failure kind aborts with exit code 1 and a structured report on
    // standard error, without running past the failure point.
    let cases: &[(&str, &str, &[&str])] = &[
        (
            "failure_panic",
            r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

pub fn main() -> Nil {
  println("before")
  panic as "boom"
}
"#,
            &["runtime error: panic", "boom", "failure_panic.main:"],
        ),
        (
            "failure_todo",
            r#"pub fn main() -> Nil {
  todo
}
"#,
            &["runtime error: todo", "This has not yet been implemented"],
        ),
        (
            "failure_let_assert",
            r#"fn broken() -> Result(Int, String) {
  Error("nope")
}

pub fn main() -> Nil {
  let assert Ok(_) = broken() as "wanted a success"
  Nil
}
"#,
            &["runtime error: let assert", "wanted a success"],
        ),
        (
            "failure_assert",
            r#"pub fn main() -> Nil {
  assert 1 == 2
  Nil
}
"#,
            &["runtime error: assert", "Assertion failed"],
        ),
    ];

    for (name, source, expectations) in cases {
        let project = TestProject::new(name, source);
        let output = project.run();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.code(),
            Some(1),
            "{name} should exit with code 1.
stdout: {stdout}
stderr: {stderr}"
        );
        for expectation in *expectations {
            assert!(
                stderr.contains(expectation),
                "{name}: missing `{expectation}` on stderr.
stderr: {stderr}"
            );
        }
        assert!(
            !stdout.contains("unreachable"),
            "{name} ran past the failure point.
stdout: {stdout}"
        );
    }

    // The panic case still runs the code before the failure.
    let project = TestProject::new(
        "failure_panic",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

pub fn main() -> Nil {
  println("before")
  panic as "boom"
}
"#,
    );
    let output = project.run();
    assert!(String::from_utf8_lossy(&output.stdout).contains("before"));
}

#[test]
fn strings() {
    let project = TestProject::new(
        "strings",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

@external(native, "runtime", "print_bool")
pub fn print_bool(value: Bool) -> Nil

@external(native, "runtime", "gleam_native_string_length")
fn length(string: String) -> Int

@external(native, "runtime", "gleam_native_string_compare")
fn compare(left: String, right: String) -> Int

@external(native, "runtime", "gleam_native_string_uppercase")
fn uppercase(string: String) -> String

@external(native, "runtime", "gleam_native_string_reverse")
fn reverse(string: String) -> String

@external(native, "runtime", "gleam_native_string_slice")
fn slice(string: String, start: Int, length: Int) -> String

@external(native, "runtime", "gleam_native_string_split")
fn split(string: String, on: String) -> List(String)

@external(native, "runtime", "gleam_native_string_pop_grapheme")
fn pop_grapheme(string: String) -> Result(#(String, String), Nil)

@external(native, "runtime", "gleam_native_string_contains")
fn contains(string: String, needle: String) -> Bool

@external(native, "runtime", "gleam_native_string_trim")
fn trim(string: String) -> String

@external(native, "runtime", "gleam_native_int_to_string")
fn int_to_string(value: Int) -> String

fn join(words: List(String), separator: String) -> String {
  case words {
    [] -> ""
    [word] -> word
    [word, ..rest] -> word <> separator <> join(rest, separator)
  }
}

fn shout_each(string: String) -> String {
  // Runtime-built results destructure in ordinary Gleam patterns.
  case pop_grapheme(string) {
    Ok(#(head, rest)) -> uppercase(head) <> shout_each(rest)
    Error(Nil) -> ""
  }
}

pub fn main() -> Nil {
  print_int(length("héllo"))
  print_int(compare("apple", "banana"))
  println(uppercase("héllo"))
  println(reverse("stressed"))
  println(slice("héllo wörld", 6, 5))
  println(join(split("a,b,c", ","), " + "))
  println(shout_each("héllo"))
  print_bool(contains("wibble", "bb"))
  println(trim("   tidy   "))
  println(int_to_string(9223372036854775808))
}
"#,
    );

    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "gleam run failed.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let expected = "5
-1
HÉLLO
desserts
wörld
a + b + c
HÉLLO
True
tidy
9223372036854775808
";
    assert!(
        stdout.contains(expected),
        "unexpected output.
stdout: {stdout}"
    );
}

#[test]
fn arguments_and_exit_codes() {
    let project = TestProject::new(
        "arguments",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "gleam_native_start_arguments")
fn start_arguments() -> List(String)

@external(native, "runtime", "gleam_native_exit")
fn exit(code: Int) -> Nil

fn print_each(arguments: List(String)) -> Int {
  case arguments {
    [] -> 0
    [argument, ..rest] -> {
      println(argument)
      1 + print_each(rest)
    }
  }
}

pub fn main() -> Nil {
  let count = print_each(start_arguments())
  // Exit with the number of arguments as the code.
  exit(count)
  println("unreachable")
}
"#,
    );

    let output = project.run_with_arguments(&["alpha", "beta", "gamma"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        output.status.code(),
        Some(3),
        "expected exit code 3.
stdout: {stdout}
stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("alpha
beta
gamma
"),
        "unexpected output.
stdout: {stdout}"
    );
    assert!(!stdout.contains("unreachable"));

    // Without arguments the program exits zero.
    let output = project.run();
    assert_eq!(output.status.code(), Some(0));
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

#[test]
fn tail_recursion() {
    // Ten million tail-recursive iterations, both direct and through an
    // accumulator over a list: genuine tail calls keep the stack constant
    // where plain calls would overflow long before finishing.
    let project = TestProject::new(
        "tail_recursion",
        r#"@external(native, "runtime", "println")
pub fn println(text: String) -> Nil

@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn count(n: Int, limit: Int) -> Int {
  case n >= limit {
    True -> n
    False -> count(n + 1, limit)
  }
}

fn build(n: Int, acc: List(Int)) -> List(Int) {
  case n {
    0 -> acc
    _ -> build(n - 1, [n, ..acc])
  }
}

fn sum(list: List(Int), total: Int) -> Int {
  case list {
    [] -> total
    [first, ..rest] -> sum(rest, total + first)
  }
}

pub fn main() -> Nil {
  print_int(count(0, 10_000_000))
  print_int(sum(build(100_000, []), 0))
  println("done")
}
"#,
    );
    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "tail recursion should finish.
stdout: {stdout}
stderr: {stderr}"
    );
    assert!(stdout.contains("10000000"), "stdout: {stdout}");
    assert!(stdout.contains("5000050000"), "stdout: {stdout}");
    assert!(stdout.contains("done"), "stdout: {stdout}");
}

#[test]
fn stack_overflow() {
    // Deep recursion that is not in tail position cannot run in constant
    // stack; it must fail with a comprehensible report, not a raw crash.
    let project = TestProject::new(
        "stack_overflow",
        r#"@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn deep(n: Int) -> Int {
  case n {
    0 -> 0
    _ -> 1 + deep(n - 1)
  }
}

pub fn main() -> Nil {
  print_int(deep(100_000_000))
}
"#,
    );
    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "stack overflow should exit with code 1.
stdout: {stdout}
stderr: {stderr}"
    );
    assert!(
        stderr.contains("runtime error: stack overflow"),
        "stderr: {stderr}"
    );
}

#[test]
fn configured_stack_size() {
    // A recursion depth that fits comfortably in the default gigabyte must
    // overflow when the project configures a small stack.
    let project = TestProject::with_config_extras(
        "configured_stack_size",
        r#"@external(native, "runtime", "print_int")
pub fn print_int(value: Int) -> Nil

fn deep(n: Int) -> Int {
  case n {
    0 -> 0
    _ -> 1 + deep(n - 1)
  }
}

pub fn main() -> Nil {
  print_int(deep(200_000))
}
"#,
        "\n[native]\nstack_size_megabytes = 4\n",
    );
    let output = project.run();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(1),
        "a 4 MB stack should overflow.
stdout: {stdout}
stderr: {stderr}"
    );
    assert!(
        stderr.contains("runtime error: stack overflow"),
        "stderr: {stderr}"
    );
}
