pub const pi_value: Bool = False
pub const euler_value: Float = 2.0

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, default: Int, v0: List(Int)) -> Bool {
constructor
}

fn f1(m: Float) -> Bool {
f0(True, case fn(v1, v2) { "a" }(0, False) {
    inner | "bc" <> inner -> 100 - 0
    inner -> 4 |> spin(4 - 100)
    b -> fn(v3) { 100 }("")
  }, {
    let m = m *. m
    let length = fn(v4, v5) { v4 }(1.5, 2)
    []
  })
}

fn new(v6: Float) -> Int {
{
    let new = 0 - 10
    let constructor = case fn(v7, v8) { new }("b", "constructor"), "b" <> "a" {
      _, "ab" -> "res"
      2, "res" -> "abc"
      _, _ -> "constructor"
    }
    spin(new, new)
  }
}

pub fn main() {
  let euler_value = ""
  echo {
    let pi_value = euler_value
    let prototype = "data"
    case <<"x":utf8>> {
      <<"a":utf8>> -> fn(v9, v10) { [100, 100] }(0.25, "a")
      <<3:16, _:8, "abc":utf8>> -> [0]
      _ -> [1, 5]
    }
  }
  echo {
    let rest = {
      {
        let arguments = 1.0
        euler_value
      }
    } <> {
      fn(v11, v12) { "constructor" }(0.25, 1.0)
    }
    let pi_value = {
      fn(v13) { euler_value }("")
    } == {
      {
        let item = 0.1
        "bc"
      }
    }
    case euler_value != euler_value {
      True | False -> {
        let constructor = pi_value
        [2]
      }
      True as whole -> [100, 1]
      v14 -> {
        let euler_value = 10.0
        []
      }
    }
  }
  echo {
    0.1
  } /. {
    10.0
  }
  echo {
    let euler_value = [5, 0]
    let euler_value = new(0.5) + 7
    True
  }
}
