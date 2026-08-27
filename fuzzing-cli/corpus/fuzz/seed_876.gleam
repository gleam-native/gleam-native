pub const tag_value: Float = 0.0
pub const seed_value: Int = 5
pub const pi_value: Int = 4

pub type V0 {
  Some(value: String, inner: List(Int))
  Cv1(value: Float, inner: Bool)
  Cv2(String, Float)
}

pub type V3 {
  Cv4
}

pub type Promise {
  Cv5
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn extends(v6: V0) -> Bool {
case "data" <> "x" {
    "res" -> case {
        let v6 = True
        "x"
      }, {
        let class = 100
        let x = "abc"
        x
      } {
      _, _ -> True
      "b" as whole, arguments -> {
        let arguments = [7]
        let prototype = 0.5
        True
      }
    }
    "a" -> case "data" {
      inner -> True
      b -> {
        let b = "x"
        True
      }
    }
    _ -> {
      4 |> spin(100)
    } > {
      2 * 3
    }
  }
}

pub fn main() {
  let prototype = 42
  let tag_value = 0.25
  echo case 5 + seed_value, "abc" <> "constructor" {
    _, "b" -> [2]
    5, "data" -> case False {
      True -> [3]
      True | True -> []
      v7 -> []
    }
    v8, v9 -> case [10] {
      [constructor, ..rest] if constructor <= 4 -> fn(v10, v11) { [100] }(True, "abc")
      [_] as whole -> []
      _ -> [7]
    }
  }
  echo 10 != {
    case pi_value |> spin(prototype + pi_value), <<"constructor":utf8, "abc":utf8, "b":utf8>> {
      0, <<"abc":utf8>> -> prototype - 100
      _, <<_:8, _:utf8, "data":utf8>> as whole -> 2 |> spin({
        let prototype = ""
        seed_value
      })
      _, _ -> 42
    }
  }
  echo [7, 42]
}
