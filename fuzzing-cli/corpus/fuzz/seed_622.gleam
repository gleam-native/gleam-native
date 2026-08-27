pub const pi_value: Int = 42
pub const golden_value: Float = 0.0
pub const limit_value: Int = 100

pub type V0 {
  Number(value: String, inner: Int)
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3
  Cv4
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: V0) -> List(Int) {
case "x" <> "x", "data" {
    "res" <> rest, "abc" -> case Cv4 {
      item -> {
        let y = 0.0
        let l = True
        [4, 100]
      }
      b -> fn(v5) { [] }(100.0)
      _ | Cv4 -> [5, 2]
    }
    "a" as whole, _ -> [42]
    _, v6 -> case fn(v7, v8) { 10 }(True, False) {
      0 -> fn(v9, v10) { [] }(10.0, 10.0)
      a -> {
        let delete = True
        let this_ = a
        [0]
      }
    }
  }
}

fn f1(n: String, v11: Float) -> Int {
case <<"x":utf8>> {
    <<"x":utf8>> -> spin(5, 5)
    <<"abc":utf8, 2:8, 7:16>> -> 42
    v12 -> {
      fn(v13) { 2 }("a")
    } % 7
  }
}

fn static(v14: Bool, v15: String, v16: List(Int)) -> List(Int) {
{
    let n = v16
    let n = case [10], fn(v17) { False }(False) {
      [0, 2, ..], False -> 1.5
      [6, ..rest], n -> 0.0
      [_], True -> {
        3.14
      } -. {
        10.0
      }
      v18, _ -> fn(v19) { 0.5 }(True)
    }
    f0(Number("a", 7))
  }
}

pub fn main() {
  let pi_value = {
    let m = Cv1([]) |> f0()
    pi_value - pi_value
  }
  echo fn(v20, v21) { case [42, 1], pi_value {
    [], _ -> 10
    [x], 2 -> 7 + 1
    v22, v23 -> 1 + 10
  } }(False, "abc")
  echo "constructor"
}
