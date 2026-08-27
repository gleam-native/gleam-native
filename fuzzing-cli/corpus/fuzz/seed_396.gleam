pub const pi_value: String = "constructor"

pub type Symbol {
  Cv0(value: String, inner: String)
  Cv1(List(Int))
  Cv2
}

fn f0(v3: Float) -> Int {
case "data" <> "abc" {
    "b" | "res" -> case <<"b":utf8>> {
      <<rest:16, _:utf8>> if rest <= 8 -> 3
      <<_:utf8, 1:1, _:little-unsigned-16>> -> 100 - 1
      _ -> 42 + 42
    }
    a -> case "a" {
      "x" <> _ | "abc" -> fn(v4) { 100 }("x")
      _ -> 0
    }
  }
}

fn f1(v5: Float, v6: #(Bool, List(Int))) -> Bool {
True
}

pub fn main() {
  echo pi_value <> {
    {
      let class = {
        let item = 0.5
        True
      }
      let rest = "x"
      "x" <> rest
    }
  }
}
