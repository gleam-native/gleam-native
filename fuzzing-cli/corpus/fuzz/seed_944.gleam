pub const pi_value: Bool = True
pub const limit_value: Int = 1

pub type Record {
  Record
  Cv0
  Error(value: String, inner: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: String, v1: String) -> Bool {
walk([10, 0], 100) < {
    case [0] {
      [class] -> 2
      [constructor, 1, ..] -> constructor * 2
      [] as whole -> walk(whole, 3)
      v2 -> 5
    }
  }
}

fn f1(value: List(Int), new: Bool) -> Int {
0
}

fn f2(v: Record) -> List(Int) {
case [10, 0] |> walk(10) {
    b -> case {
        let b = "constructor"
        42
      }, False {
      _, True -> fn(v3, v4) { [3, 100] }("abc", 10)
      0, _ -> {
        let b = []
        [10]
      }
      _, v5 -> [4, 3]
    }
    item -> case fn(v6, v7) { "" }(True, False) {
      default -> [42, 42]
      "x" | "constructor" -> [4, 3]
    }
  }
}

pub fn main() {
  let limit_value = 100.0
  let x = "ab" |> f0("data")
  echo "x"
  echo fn(v8) { case #(True, 10) {
    item -> 100 >= 7
    b -> f0("x", "x")
  } }(2.0)
  echo {
    case 100 + 100, #(2.0, "ab") {
      4, #(pi_value, "x" <> _) as whole -> limit_value /. {
        10.0
      }
      2 as whole, #(n, _) -> n
      v9, v10 -> limit_value
    }
  } +. limit_value
  echo {
    case "constructor", "constructor" <> "res" {
      "x", v11 -> fn(v12) { 0.1 }(False)
      "data" <> _, v13 -> {
        let class = limit_value
        limit_value
      }
      _, _ -> {
        1.0
      } /. {
        3.14
      }
    }
  } /. {
    1.0
  }
}
