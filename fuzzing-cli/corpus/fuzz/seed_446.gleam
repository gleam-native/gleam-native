pub const pi_value: String = "constructor"
pub const euler_value: String = "res"

pub type V0 {
  Error(value: String, inner: String)
  Cv1
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Int) -> Float {
case spin(v2, v2), Cv1 {
    6, Cv1 -> fn(v3, v4) { fn(v5) { 1.0 }(5) }(5, 0.0)
    9, Error("res", _) -> case [4], Error("data", "data") {
      [b, ..rest], _ if b % 2 == 0 -> {
        3.14
      } +. {
        2.0
      }
      [], v2 -> {
        1.0
      } +. {
        0.25
      }
      v6, _ -> {
        0.1
      } +. {
        0.25
      }
    }
    _, _ -> case Cv1 {
      _ -> 2.0
      _ | Cv1 -> {
        2.0
      } +. {
        100.0
      }
      Error(constructor, _) -> {
        0.25
      } /. {
        10.0
      }
    }
  }
}

fn f1(arguments: Float) -> List(Int) {
case "a" <> "res" {
    _ | "ab" -> [7]
    "bc" | "constructor" <> _ -> [100]
    item -> []
  }
}

fn export(v7: Int) -> Int {
5
}

pub fn main() {
  let class = {
    2 |> export()
  } <= export(42)
  echo 100
  echo 1
  echo fn(v8) { [5, 42] }("constructor")
  echo case <<"abc":utf8, "a":utf8>>, 1 {
    <<42:16, _:8>>, 3 -> case 4, #("res", False) {
      _, #("b", False) as whole -> {
        10.0
      } <=. {
        0.1
      }
      _, #("bc", _) as whole -> {
        let whole = False
        let self_ = pi_value
        class
      }
      7, #("data", True) as whole -> class
      v9, v10 -> False
    }
    <<7:16>>, 8 -> class || False
    _, 5 -> False
    v11, v12 -> class
  }
}
