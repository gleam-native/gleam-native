pub const seed_value: Float = 1.0
pub const euler_value: Int = 4

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Float, new: Bool, prototype: List(Int)) -> Float {
{
    case fn(v0) { "data" }(True) {
      b -> 1.0
      _ | "constructor" <> _ -> constructor
      inner -> constructor
    }
  } +. {
    case {
        let new = True
        "ab"
      } {
      "abc" | "data" -> constructor
      "res" as whole if whole != "a" || whole == "res" -> 10.0
      item | "b" <> item -> {
        2.0
      } +. {
        0.1
      }
    }
  }
}

pub fn main() {
  let rest = {
    {
      let default = []
      let self_ = "data"
      self_
    }
  } <> "x"
  let seed_value = case euler_value - euler_value {
    b -> []
    v1 -> [4]
  }
  echo {
    {
      fn(v2, v3) { rest }(1.5, False)
    } <> rest
  } <> {
    {
      let this_ = 100
      let this_ = {
        let this_ = [2]
        seed_value
      }
      rest
    }
  }
  echo 100.0
}
