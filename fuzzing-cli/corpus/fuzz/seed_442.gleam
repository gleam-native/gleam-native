pub const pi_value: Float = 3.14
pub const euler_value: Int = 3
pub const limit_value: Int = 10

pub type Promise {
  Record
  Cv0
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: #(Float, Bool), v2: Bool) -> Bool {
{
    case <<"bc":utf8, 2:1>> {
      <<100:16, _:little-signed-8>> -> {
        0.0
      } /. {
        3.14
      }
      <<4:8, 5:16>> -> 100.0
      _ -> 100.0
    }
  } <. {
    fn(v3, v4) { {
      let v2 = 100
      v3
    } }(10.0, False)
  }
}

fn new(length: Int, v5: Int, z: Promise) -> Float {
10.0
}

pub fn main() {
  let v = case <<"a":utf8>> {
    <<3:4, 1:8, 100:16>> -> 2.0
    _ -> pi_value +. {
      100.0
    }
  }
  let limit_value = {
    let length = {
      let z = "ab"
      False
    }
    let v = 4
    v + 42
  }
  echo case fn(v6) { False }("bc") {
    False -> {
      let z = fn(v7) { "ab" }(True)
      let euler_value = z
      {
        let m = pi_value
        "a"
      }
    }
    True -> {
      fn(v8, v9) { v8 }("b", "bc")
    } <> {
      "" <> "constructor"
    }
  }
  echo {
    let arguments = fn(v10) { 42 }("res")
    case [] {
      [2] -> 2
      [] -> [7] |> walk(7)
      _ -> 4
    }
  }
}
