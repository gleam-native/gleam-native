pub const pi_value: Int = 0
pub const tag_value: Bool = False
pub const golden_value: String = "a"

pub type Record {
  Cv0(value: String, inner: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(prototype: Int) -> String {
{
    case <<"a":utf8>> {
      <<"constructor":utf8>> -> "x"
      _ -> "ab"
    }
  } <> {
    "ab" <> {
      fn(v1, v2) { v2 }(10.0, "")
    }
  }
}

fn default(default: List(Int), new: Int) -> String {
"data"
}

pub fn main() {
  let pi_value = 42
  let new = case "x" {
    "ab" | "" <> _ -> []
    "bc" <> constructor -> {
      let length = [10, 7]
      let pi_value = tag_value
      length
    }
    _ -> [10, 5]
  }
  echo case golden_value {
    "data" -> {
      fn(v3, v4) { 42 }(1.5, "a")
    } - pi_value
    _ | "ab" -> 2
  }
}
