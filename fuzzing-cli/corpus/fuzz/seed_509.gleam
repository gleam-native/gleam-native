pub const seed_value: String = "abc"
pub const limit_value: Float = 3.14
pub const pi_value: String = "abc"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(prototype: #(Bool, Float)) -> List(Int) {
[0, 4]
}

fn f1(delete: String) -> List(Int) {
constructor(#(False, 0.0))
}

fn f2(v0: Float, v1: Int, default: Float) -> Float {
case <<1:16, 1:1>> {
    <<s:little-unsigned-8, _:big-unsigned-8, 4:8>> as whole -> {
      default -. {
        0.0
      }
    } /. {
      0.5
    }
    _ -> 1.0
  }
}

pub fn main() {
  let y = [10]
  echo [5]
  echo {
    let prototype = {
      seed_value <> seed_value
    } <> {
      pi_value <> "constructor"
    }
    let seed_value = case 10.0, [2] {
      _, [_, ..rest] -> False
      _, [constructor] -> False
      _, v2 -> fn(v3) { True }(42)
    }
    fn(v4) { prototype }("ab")
  }
  echo {
    case {
        let pi_value = 0
        let z = y
        True
      } {
      False -> 42 % 3
      True | True -> {
        let s = [2, 4]
        3
      }
      True | True -> 3
    }
  } >= 2
  echo limit_value
}
