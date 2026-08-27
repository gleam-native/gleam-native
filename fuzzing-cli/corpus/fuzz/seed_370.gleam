pub const pi_value: Bool = False
pub const seed_value: String = "res"

pub type Record {
  Cv0(value: String, inner: List(Int))
}

pub type V1 {
  None(String)
}

pub type V2 {
  Number(value: List(Int))
  Cv3(String, Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn extends(v4: List(Int)) -> Int {
100 + 4
}

fn f1(v: List(Int), length: #(String, Float), v5: String) -> Float {
case fn(v6, v7) { v6 }(100, False), 100 {
    5, 6 -> {
      {
        let v = "a"
        let length = "data"
        10.0
      }
    } +. {
      0.0
    }
    _, _ -> 0.1
  }
}

fn f2(v8: Int, acc: Bool, v9: Int) -> Float {
100.0
}

pub fn main() {
  let this_ = {
    1.5
  } *. {
    1.0
  }
  let acc = case "data" <> "abc" {
    "abc" <> _ -> pi_value
    "abc" -> pi_value
    _ -> pi_value
  }
  echo [7]
}
