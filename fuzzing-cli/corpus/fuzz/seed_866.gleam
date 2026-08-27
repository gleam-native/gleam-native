pub const golden_value: Float = 1.0
pub const euler_value: Float = 0.0
pub const limit_value: String = "bc"

pub type V0 {
  Record(value: String, inner: Int)
}

pub type V1 {
  Cv2(value: Bool)
  Cv3(List(Int), Int)
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v5: V1, arguments: V0) -> String {
{
    fn(v6, v7) { "a" }(True, 4)
  } <> "res"
}

fn f1(v8: List(Int)) -> Bool {
{
    case 3 - 2 {
      _ -> spin(42, 42)
      7 -> 100
    }
  } != 3
}

fn class(value: Float, s: Int) -> Float {
case "a" {
    "abc" <> rest as whole -> case False {
      _ | True -> {
        10.0
      } +. {
        1.0
      }
      False as whole -> 0.25
    }
    b -> 0.25
  }
}

pub fn main() {
  echo [1, 42]
  echo case Record("b", 4) {
    Record(a, _) if a != "bc" -> case 4 {
      inner -> fn(v9, v10) { inner }(True, 4)
      b -> b - b
      v11 -> v11 + v11
    }
    Record(prototype, _) -> case {
        let m = 0.1
        let delete = "constructor"
        "x"
      } {
      b | "res" <> b -> fn(v12, v13) { v12 }(3, 100.0)
      _ -> {
        let golden_value = golden_value
        let v = 0.5
        3
      }
      length | "res" <> length -> fn(v14) { 3 }("ab")
    }
    Record("constructor", 8) | Record(_, _) -> {
      0 + 100
    } - {
      0 |> spin(spin(5, 4))
    }
  }
  echo {
    let self_ = case <<1:16>>, [2] {
      <<_:utf8>>, [] -> f0(Cv3([5, 3], 4), Record("res", 100))
      <<4:8>> as whole, [2, 7, ..] -> {
        let rest = euler_value
        limit_value
      }
      _, [a] -> limit_value
      _, _ -> fn(v15, v16) { "data" }(0, "constructor")
    }
    let l = {
      {
        let v = limit_value
        ""
      }
    } <> {
      fn(v17, v18) { limit_value }(2, 0.0)
    }
    {
      "abc" <> "b"
    } <> {
      Cv2(True) |> f0(fn(v19) { Record("bc", 5) }(10.0))
    }
  }
  echo True
}
