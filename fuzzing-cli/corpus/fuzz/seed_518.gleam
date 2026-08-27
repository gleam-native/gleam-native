pub const euler_value: Int = 100

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: V0, m: String) -> Float {
case m <> "", m == "a" {
    "res" <> _, False -> case walk([2], 42) {
      2 -> {
        1.0
      } -. {
        1.0
      }
      9 as whole -> 0.25
      _ -> {
        10.0
      } +. {
        2.0
      }
    }
    "abc" <> _ as whole, False -> fn(v3) { {
      1.0
    } +. {
      1.5
    } }(5)
    v4, _ -> {
      100.0
    } -. {
      3.14
    }
  }
}

fn f1(v5: Float, default: V0) -> Float {
v5
}

fn f2(v6: String) -> Float {
0.1
}

pub fn main() {
  echo case fn(v7, v8) { v7 }("a", True) {
    b -> {
      "constructor" <> b
    } <> {
      {
        let n = "bc"
        n
      }
    }
    inner -> {
      inner <> inner
    } <> "res"
  }
}
