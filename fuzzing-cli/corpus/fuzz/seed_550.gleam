fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, rest: String, delete: Float) -> List(Int) {
[]
}

pub fn main() {
  echo case fn(v0) { 3 }(True) {
    5 -> 0.1
    7 | 2 -> {
      {
        let acc = 0.5
        acc
      }
    } -. {
      {
        0.0
      } -. {
        0.25
      }
    }
    v1 -> case [0] {
      [] -> {
        2.0
      } *. {
        0.5
      }
      [] -> {
        10.0
      } *. {
        1.5
      }
      _ -> fn(v2) { 100.0 }(0.0)
    }
  }
  echo {
    case "res" {
      "abc" <> inner -> 3
      "b" -> fn(v3) { 0 }(0.1)
      _ -> 100
    }
  } * 100
  echo False
}
