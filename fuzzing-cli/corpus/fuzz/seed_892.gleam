pub const seed_value: Float = 0.1
pub const pi_value: Float = 0.25
pub const euler_value: Int = 7

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(l: #(Bool, List(Int))) -> Float {
case <<"abc":utf8, "constructor":utf8>> {
    <<_:8>> -> case <<"b":utf8>> {
      <<_:1>> -> fn(v0, v1) { 100.0 }("bc", 3)
      <<10:16>> -> {
        let z = "a"
        10.0
      }
      _ -> {
        0.25
      } -. {
        0.25
      }
    }
    _ -> fn(v2, v3) { 2.0 }(True, "constructor")
  }
}

fn f1(v4: Int, default: String, v5: Int) -> Int {
v4
}

fn f2(rest: Int, v6: String, v7: Int) -> Int {
v7 * {
    {
      v7 + v7
    } - rest
  }
}

pub fn main() {
  echo pi_value
  echo {
    let seed_value = False
    let pi_value = {
      let rest = fn(v8) { [2, 10] }(True)
      let class = True
      constructor(#(True, [42]))
    }
    case "abc", 10 {
      "ab" <> _, 0 -> {
        let rest = "a"
        rest
      }
      "abc", 9 -> fn(v9, v10) { "abc" }(4, 7)
      v11, _ -> "b" <> "bc"
    }
  }
}
