pub const golden_value: Float = 0.25
pub const euler_value: Int = 5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(default: Bool) -> Bool {
case {
      let default = "b"
      let default = 3
      #(10.0, 100)
    }, fn(v0, v1) { "constructor" }(False, 10) {
    #(100.0, 4), "x" -> default
    #(default, 7) as whole, "bc" <> rest -> case False, [100] {
      False, [a, ..rest] -> False || True
      True, [_, constructor, ..] -> {
        let s = 7
        True
      }
      rest, [3, ..tail] -> "a" != "abc"
      v2, v3 -> {
        10.0
      } != {
        1.5
      }
    }
    #(0.5, 0 as whole) as it, "ab" -> False
    _, _ -> default
  }
}

pub fn main() {
  let default = {
    let pair = {
      0.5
    } != golden_value
    let n = constructor(pair)
    {
      let z = "constructor"
      let n = 0.5
      ""
    }
  }
  let l = 10.0
  echo {
    let euler_value = []
    let v = 5
    case "bc" {
      inner -> fn(v4, v5) { l }(True, False)
      "bc" -> l
    }
  }
}
