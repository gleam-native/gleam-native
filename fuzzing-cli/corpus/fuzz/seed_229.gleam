pub const euler_value: Float = 10.0
pub const tag_value: Bool = True
pub const seed_value: Float = 1.0

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: Int, v1: String) -> Int {
{
    case {
        let default = v1
        let v1 = "res"
        v1
      }, fn(v2, v3) { [] }(0, 42) {
      "bc", [_, ..rest] -> {
        let m = v0
        rest
      }
      "" <> rest as whole, [_, ..tail] as it -> fn(v4) { it }(1)
      v5, v6 -> {
        let v = v6
        let constructor = constructor
        v6
      }
    }
  } |> walk({
    let prototype = "abc"
    2
  })
}

fn f1(s: String) -> List(Int) {
case {
      let s = True
      s
    } {
    False | False -> case s <> s {
      "abc" <> a -> [5, 7]
      "x" <> rest | "bc" <> rest -> {
        let s = 10.0
        [2]
      }
      _ -> [2]
    }
    False -> case "bc" <> s {
      s -> []
      "b" -> fn(v7) { [7, 100] }(100.0)
    }
    _ -> {
      let s = 100
      {
        let s = 10.0
        [2]
      }
    }
  }
}

fn f2(n: Bool, new: Float, self_: Int) -> Int {
2
}

pub fn main() {
  let seed_value = 10.0
  echo {
    let rest = "abc"
    seed_value
  }
}
