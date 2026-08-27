pub const euler_value: Bool = True
pub const golden_value: String = "bc"
pub const seed_value: Int = 10

pub type V0 {
  Ok(value: String, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(y: #(List(Int), Bool), v1: #(Bool, List(Int))) -> Int {
case 7 + 42, [] {
    _, [1, ..rest] -> case <<10:4, "bc":utf8, 3:16>> {
      <<2:8>> as whole -> spin(10, 10)
      <<10:8>> -> spin(3, 2)
      <<_:utf8>> as whole -> 4 * 7
      _ -> 1
    }
    6, [] -> spin(100 |> spin(5 * 0), spin(10, 10))
    _, _ -> 5
  }
}

fn f1(v2: Float) -> Int {
{
    {
      2 * 2
    } + {
      1 + 2
    }
  } + 42
}

pub fn main() {
  let acc = case seed_value + seed_value, seed_value - seed_value {
    _, 5 -> "abc" <> "b"
    0, _ -> fn(v3) { "data" }(7)
    _, _ -> golden_value
  }
  let new = case golden_value {
    _ -> seed_value < 1
    "res" <> _ -> {
      100.0
    } == {
      100.0
    }
  }
  echo {
    case acc, "b" <> golden_value {
      "bc", "data" -> 7
      "ab" as whole, "x" -> {
        10.0
      } |> f1()
      v4, _ -> seed_value % 4
    }
  } + 100
  echo case golden_value, fn(v5, v6) { Ok("b", 42) }(0, False) {
    _, Ok(v7, v8) if v7 == "constructor" -> case spin(v8, seed_value) {
      0 | 2 -> 5 - seed_value
      4 -> fn(v9) { 5 }(True)
      _ -> v8 * v8
    }
    _, v10 -> f0(fn(v11) { #([], True) }(""), #(True, [10]))
  }
  echo 1.0
  echo [100, 100]
}
