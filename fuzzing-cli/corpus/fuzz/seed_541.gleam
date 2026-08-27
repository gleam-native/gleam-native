pub const euler_value: Bool = False
pub const tag_value: String = ""
pub const pi_value: String = ""

pub type V0 {
  Number(value: String, inner: String)
  Cv1
  None(List(Int))
}

fn f0(value: V0, v2: List(Int)) -> List(Int) {
v2
}

pub fn main() {
  let tag_value = case <<"a":utf8>> {
    <<0:8>> as whole -> 0
    <<"":utf8, "b":utf8>> -> fn(v3) { 5 }(0.1)
    _ -> 7 + 3
  }
  echo 4
  echo case "constructor" <> "" {
    _ | "x" -> case pi_value <> pi_value, "x" {
      "abc", "b" -> {
        let pi_value = 7
        let l = "ab"
        1.5
      }
      "data", "" <> _ -> 10.0
      euler_value, "a" -> fn(v4, v5) { 100.0 }(True, True)
      _, v6 -> fn(v7, v8) { v8 }(2, 0.0)
    }
    item -> case Cv1, "bc" {
      v9, delete -> 1.5
      v10, _ -> 100.0
    }
  }
  echo case tag_value {
    this_ -> case pi_value {
      "res" <> constructor if constructor == "b" -> fn(v11, v12) { 1.0 }(100, 1.0)
      "a" -> 0.0
      _ -> fn(v13, v14) { 2.0 }("b", 10.0)
    }
    1 -> case fn(v15, v16) { euler_value }(10.0, "data") {
      True | True -> {
        100.0
      } *. {
        3.14
      }
      v -> {
        0.1
      } -. {
        100.0
      }
      a -> 0.25
    }
  }
  echo pi_value
}
