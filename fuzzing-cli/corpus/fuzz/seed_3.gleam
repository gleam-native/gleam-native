pub const seed_value: Bool = True
pub const tag_value: Float = 1.5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(String, Float), delete: String, prototype: Int) -> String {
"res"
}

fn f1(default: Float) -> List(Int) {
[0]
}

pub fn main() {
  let seed_value = 1
  echo case "data" {
    inner | "constructor" <> inner -> case "ab" <> "x" {
      _ | "x" <> _ -> False
      "a" | "bc" -> !True
    }
    "constructor" <> constructor -> case fn(v0) { constructor }(True) {
      constructor -> True
      item -> False
      "data" <> _ -> False
    }
    "ab" -> True
  }
  echo case {
      let value = False
      let s = 1
      "abc"
    }, False {
    "res", _ -> {
      fn(v1) { 0 }(True)
    } - {
      0 + seed_value
    }
    "constructor" as whole, False if whole == "constructor" && whole == "abc" -> case walk([3, 42], 4) {
      8 | 9 -> 0
      7 -> seed_value + seed_value
      item -> fn(v2) { 42 }(False)
    }
    "data", _ -> fn(v3) { seed_value - seed_value }("a")
    _, _ -> seed_value
  }
  echo case tag_value {
    0.5 -> case f0(#("", 2.0), "bc", 5), {
        let length = 1
        ""
      } {
      acc, v -> {
        let z = acc
        0
      }
      "bc" <> _, v -> walk([4, 5], 2)
    }
    inner -> seed_value
  }
  echo tag_value
}
