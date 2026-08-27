pub const pi_value: Int = 4

fn f0(m: String) -> Float {
case <<3:1>>, <<"x":utf8>> {
    <<"":utf8>>, <<"b":utf8>> -> case True, "ab" {
      _, "abc" -> 100.0
      False, _ -> 10.0
      False, "b" as whole -> fn(v0, v1) { 0.25 }(2.0, 4)
      _, _ -> 10.0
    }
    _, <<_:16, z:little-signed-4, "abc":utf8>> -> 2.0
    v2, v3 -> fn(v4, v5) { 1.0 }(1, "constructor")
  }
}

pub fn main() {
  echo {
    case 0.25 {
      b -> {
        let m = pi_value
        let m = []
        "constructor"
      }
      item -> {
        let constructor = ""
        constructor
      }
    }
  } |> f0()
}
