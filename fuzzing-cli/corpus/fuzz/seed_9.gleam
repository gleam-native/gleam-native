pub const seed_value: Bool = False
pub const euler_value: Int = 5
pub const golden_value: String = "data"

pub type V0 {
  Number(value: String, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(arguments: #(Float, String), l: Int, item: #(Float, Bool)) -> List(Int) {
[]
}

fn f1(class: Int, acc: String) -> Int {
100
}

fn f2(pair: Int) -> Float {
case "" <> "abc" {
    "data" <> rest as whole -> fn(v1) { {
      let whole = []
      0.0
    } }(0.0)
    inner | "ab" <> inner -> {
      let self_ = 3.14
      fn(v2, v3) { self_ }("x", 4)
    }
    "b" <> b -> case fn(v4) { 42 }(2.0) {
      9 -> {
        1.0
      } *. {
        2.0
      }
      item -> 3.14
    }
  }
}

pub fn main() {
  let v = walk([5], euler_value) - 100
  let z = f2(v)
  echo [2]
  echo {
    case "res" <> "data", f0(#(1.0, "data"), v, #(0.5, False)) {
      _, [] -> fn(v5, v6) { 10.0 }(False, "x")
      "b" <> _, [] -> z +. {
        2.0
      }
      seed_value, [2, h, ..] as whole -> 10.0
      _, _ -> f2(5)
    }
  } /. {
    0.5
  }
  echo {
    fn(v7) { 100.0 }(False)
  } *. {
    case "b", 2 {
      "ab", 1 -> {
        let rest = seed_value
        z
      }
      "x", 5 -> 2.0
      _, v8 -> z +. {
        0.25
      }
    }
  }
}
