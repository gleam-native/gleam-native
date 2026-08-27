pub const euler_value: Bool = False
pub const tag_value: Int = 4

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, m: List(Int), length: Int) -> Int {
5
}

pub fn main() {
  echo {
    case fn(v0, v1) { "x" }(False, False) {
      "abc" <> inner if inner == "bc" -> f0("ab", [7], 5)
      b -> tag_value % 2
    }
  } * tag_value
  echo case "abc" {
    "res" | "data" <> _ -> "x" <> {
      "x" <> "constructor"
    }
    "x" -> case fn(v2, v3) { #([10, 42], 3) }("ab", False) {
      #([constructor, ..rest], v4) -> "constructor"
      #([], 2) as whole -> "b"
      _ -> {
        let y = "a"
        y
      }
    }
    _ -> case "x" {
      "ab" <> rest | "b" <> rest -> fn(v5) { "" }(2)
      a | "ab" <> a -> fn(v6) { a }(False)
      "a" <> _ | "a" -> "abc" <> "constructor"
    }
  }
  echo tag_value
  echo case {
      2.0
    } +. {
      0.5
    } {
    item -> case "" <> "a" {
      "b" | "bc" <> _ -> {
        let tag_value = 2
        let constructor = "bc"
        []
      }
      _ -> [42, 7]
      "res" <> _ -> [2]
    }
    constructor -> case fn(v7, v8) { #(100, 1.5) }(True, 0), fn(v9) { "bc" }("data") {
      #(_, tag_value), _ -> [1]
      #(5, 0.5 as whole), "abc" -> []
      #(7, 1.5), "x" -> [2]
      _, _ -> [7, 100]
    }
  }
}
