pub const pi_value: Float = 0.25
pub const euler_value: Bool = True
pub const tag_value: Float = 100.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Number(value: Bool, inner: List(Int))
  Error
  Cv3(value: Int, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v4: Float) -> Bool {
case fn(v5, v6) { 2.0 }(False, "a") {
    inner -> True
    _ -> case "abc" <> "ab" {
      "a" | "constructor" <> _ -> 10 >= 100
      "data" <> rest | "data" <> rest -> fn(v7) { False }(0.0)
      "res" <> constructor -> True
      _ -> False
    }
  }
}

fn f1(v8: Int, m: V2) -> List(Int) {
[0]
}

fn f2(new: String, v9: Int, pair: Int) -> Bool {
{
    case [2, 3] {
      [x, _, ..] if x % 2 == 0 || x <= 0 -> fn(v10, v11) { v11 }(True, 0.5)
      [_, _, ..] -> fn(v12) { 0.1 }("constructor")
      [4, 2, ..] -> 0.0
      _ -> 1.5
    }
  } != {
    case #(True, 0.25) {
      #(False, 0.0) as whole -> {
        1.5
      } *. {
        0.1
      }
      #(n, 0.1 as whole) -> fn(v13) { whole }(5)
      v14 -> {
        2.0
      } *. {
        100.0
      }
    }
  }
}

pub fn main() {
  echo case 100 {
    1 -> case f2("constructor", 0, 5), tag_value {
      False, v15 -> "bc"
      True, 1.0 -> fn(v16) { "data" }(True)
      False, v -> {
        let v = []
        "x"
      }
      v17, _ -> "x"
    }
    5 -> {
      let arguments = 3.14
      let prototype = fn(v18, v19) { pi_value }(True, 3.14)
      "x"
    }
    inner -> "ab"
  }
  echo f1({
    5 + 100
  } + 100, fn(v20) { Number(False, [5]) }(3))
}
