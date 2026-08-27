pub const golden_value: String = "constructor"
pub const tag_value: String = "ab"
pub const limit_value: Bool = True

fn f0(m: String, acc: Int) -> Bool {
!{
    case acc {
      _ | 4 -> {
        let s = True
        let acc = True
        s
      }
      3 | 8 -> False
      9 -> False && True
    }
  }
}

fn f1(v0: #(Int, Bool), v1: Bool, constructor: Float) -> Int {
{
    case fn(v2) { #("bc", 3.14) }(0) {
      #(this_, _) -> 1
      #(_, 0.25) as whole -> 2 + 4
      _ -> fn(v3) { 10 }(2.0)
    }
  } * {
    fn(v4) { 5 }(1.0)
  }
}

pub fn main() {
  let self_ = f1(#(2, True), False, 0.5) % 1
  echo limit_value
  echo {
    let rest = case True, golden_value <> "bc" {
      False, "" <> rest if rest != "" || rest != "ab" -> "bc" <> tag_value
      _, _ -> golden_value <> "b"
    }
    0.0
  }
  echo {
    {
      {
        let delete = 1.5
        let delete = delete
        #(10, True)
      }
    } |> f1(limit_value, 3.14)
  } % 3
}
