fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v0: #(Bool, Int), v1: Float) -> Float {
case v1 +. {
      2.0
    } {
    _ -> {
      let constructor = "a"
      {
        let item = 5
        v1
      }
    }
    0.0 -> case constructor {
      _ -> {
        1.0
      } +. v1
      _ -> v1 -. {
        1.5
      }
      9 -> v1
    }
  }
}

pub fn main() {
  echo "ab"
  echo fn(v2) { 5 }(2.0)
  echo {
    {
      100.0
    } /. {
      0.5
    }
  } -. {
    fn(v3) { 2.0 }(3)
  }
}
