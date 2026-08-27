pub const seed_value: Float = 2.0
pub const golden_value: Bool = True
pub const euler_value: Float = 3.14

pub type Record {
  Cv0(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(prototype: String) -> Bool {
{
    let default = {
      {
        10.0
      } -. {
        2.0
      }
    } +. {
      {
        let prototype = "b"
        0.25
      }
    }
    let this_ = [10, 3] |> walk(5 % 4)
    True
  }
}

pub fn main() {
  let golden_value = case fn(v1) { "abc" }("res") {
    constructor -> [1, 42]
    "bc" | "a" -> {
      let s = golden_value
      let pair = "data"
      [3, 7]
    }
  }
  let l = {
    let euler_value = {
      let y = 7
      let new = seed_value
      "abc"
    }
    let seed_value = seed_value
    euler_value <> euler_value
  }
  echo 7
}
