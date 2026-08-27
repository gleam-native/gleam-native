pub const golden_value: Bool = True

pub type Promise {
  Cv0(value: String, inner: List(Int))
  Cv1(Bool)
}

fn delete(v2: #(Bool, Float)) -> List(Int) {
[7]
}

pub fn main() {
  let item = "ab"
  echo case #(True, 2) {
    #(True, 5) -> 10.0
    #(_, 0) -> case item {
      "b" <> rest | "ab" <> rest -> 10.0
      _ | "ab" -> 1.0
      "constructor" -> {
        1.5
      } +. {
        1.5
      }
    }
    _ -> case #(100.0, "abc"), {
        1.0
      } *. {
        0.5
      } {
      #(3.14, "" <> _) as whole, _ -> {
        let item = 2
        1.0
      }
      #(x, "constructor"), _ -> x +. x
      _, v3 -> 2.0
    }
  }
  echo case fn(v4, v5) { v4 }(4, "res") {
    8 as whole if whole > 3 -> 1.5
    item -> 3.14
  }
  echo "res"
}
