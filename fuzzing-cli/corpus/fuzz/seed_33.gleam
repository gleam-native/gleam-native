pub const golden_value: Int = 0
pub const seed_value: Float = 3.14
pub const pi_value: String = ""

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v0: Int, pair: Bool) -> Bool {
False
}

fn f1(class: Int) -> Bool {
False
}

pub fn main() {
  let x = case "abc", 2 {
    "b" <> rest, constructor -> []
    "constructor" <> rest, 1 -> [100, 42]
    "constructor", 8 -> [7]
    _, _ -> [4]
  }
  let length = {
    1.5
  } +. {
    3.14
  }
  echo {
    "ab" <> ""
  } <> {
    {
      fn(v1, v2) { "a" }("abc", "a")
    } <> pi_value
  }
  echo False
}
