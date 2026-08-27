pub const euler_value: String = "a"
pub const limit_value: Float = 1.0

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: Int, this_: Int) -> String {
""
}

fn f1(v1: Float) -> Bool {
{
    0.1
  } <. v1
}

pub fn main() {
  let euler_value = euler_value
  let z = case True |> f0(4 - 42, fn(v2) { 5 }(100.0)) {
    item -> item <> "a"
    "b" | "res" -> euler_value <> euler_value
  }
  echo case {
      let s = 0
      let limit_value = [2]
      True
    }, True {
    False, False as whole if whole -> {
      {
        let euler_value = whole
        let this_ = limit_value
        this_
      }
    } /. {
      2.0
    }
    True, True as whole -> {
      fn(v3) { 0.0 }(0)
    } *. limit_value
    v4, True -> {
      {
        1.5
      } /. {
        2.0
      }
    } -. {
      100.0
    }
    v5, v6 -> case 4, 5 * 4 {
      v, 6 as whole -> {
        let prototype = [10, 1]
        let rest = limit_value
        rest
      }
      _, 9 -> limit_value
      _, v7 -> {
        let l = 4
        let prototype = "res"
        0.25
      }
    }
  }
}
