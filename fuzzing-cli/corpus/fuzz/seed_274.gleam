pub const seed_value: String = "constructor"
pub const golden_value: String = "abc"
pub const limit_value: Bool = True

pub type Record {
  Cv0(value: String, inner: String)
}

pub type V1 {
  Cv2(Int, Bool)
  Cv3
}

fn f0(pair: Int) -> String {
"bc"
}

pub fn main() {
  echo {
    case 10 * 4, f0(7) {
      _, "constructor" <> rest if rest == "x" -> "b"
      _, "abc" -> seed_value
      v4, v5 -> {
        let v = 5
        let class = 1.0
        v5
      }
    }
  } <> golden_value
  echo case {
      10.0
    } -. {
      2.0
    }, f0(4) {
    0.1, "a" <> _ as whole if whole == "abc" -> 2.0
    1.0 as whole, "abc" <> _ -> whole
    v6, _ -> case fn(v7, v8) { 0.1 }(2, True), <<"a":utf8>> {
      3.14 as whole, <<"":utf8>> -> v6
      this_, _ -> 0.5
    }
  }
}
