pub const tag_value: String = "x"
pub const seed_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(item: Bool) -> Bool {
case [] {
    [] -> case "bc", {
        let constructor = 1
        #(False, "abc")
      } {
      "abc" <> rest, #(_, "bc") as whole -> True
      _, #(_, "ab") -> !True
      _, v0 -> item
    }
    [item, b, ..] -> True
    _ -> 100 >= walk([], 2)
  }
}

pub fn main() {
  let prototype = fn(v1) { v1 <> tag_value }("data")
  let prototype = {
    let y = 0.5
    let this_ = fn(v2) { True }(3)
    y *. y
  }
  echo 0.0
  echo case {
      let y = seed_value
      let tag_value = 10
      ""
    } {
    "abc" -> "res"
    "x" <> rest | "abc" <> rest -> rest
    "x" | "constructor" -> tag_value
    _ -> case "x" <> tag_value, [] {
      "bc", [4, ..rest] -> tag_value
      "data", [b, 8, ..] as whole -> fn(v3) { tag_value }(4)
      "bc" <> rest, [a, ..tail] -> "b"
      _, _ -> fn(v4) { tag_value }(0)
    }
  }
  echo 7 + {
    10 + {
      2 + 4
    }
  }
  echo case fn(v5) { tag_value }(0.0) {
    a | "ab" <> a -> fn(v6) { 42 }("res")
    "b" -> case 0.0 {
      _ -> 1
      0.0 -> 4
      0.25 -> 10
    }
  }
}
