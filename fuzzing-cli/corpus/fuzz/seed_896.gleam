pub const euler_value: Bool = True
pub const seed_value: Bool = False
pub const golden_value: Int = 42

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v0: Int, value: Float) -> Bool {
{
    let v0 = "res"
    let self_ = "b"
    {
      v0 <> self_
    } != {
      self_ <> v0
    }
  }
}

fn f1(this_: List(Int), v1: Float, v2: Int) -> List(Int) {
{
    let default = {
      fn(v3, v4) { v1 }(0.0, False)
    } +. {
      v1 *. v1
    }
    case v2 |> f0({
        0.0
      } -. v1), Record {
      True as whole, Record -> this_
      _, Record -> {
        let class = this_
        [4, 1]
      }
      False, Record -> this_
      _, _ -> this_
    }
  }
}

pub fn main() {
  echo golden_value
  echo case "b" <> "constructor" {
    "" <> _ -> case golden_value {
      0 -> fn(v5) { "data" }(0.0)
      7 as whole -> "a" <> "data"
      v6 -> "b" <> "b"
    }
    b | "b" <> b -> "b"
    euler_value -> {
      {
        let prototype = euler_value
        prototype
      }
    } <> "bc"
  }
}
