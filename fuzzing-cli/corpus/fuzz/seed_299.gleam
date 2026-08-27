pub const golden_value: Int = 4
pub const tag_value: Bool = True
pub const seed_value: Float = 100.0

pub type V0 {
  Ok(value: String, inner: Float)
  Cv1(Int, Float)
}

pub type V2 {
  Cv3(value: List(Int))
  Error(Float, List(Int))
  Cv4(String, Bool)
}

fn f0(v5: List(Int), this_: #(Float, Bool)) -> Int {
{
    case 5, 2 {
      9, _ -> {
        let m = 0.1
        4
      }
      7, 2 -> 5 * 4
      _, v6 -> 4 + v6
    }
  } % 7
}

fn f1(v7: String, m: Int, arguments: Bool) -> Bool {
False
}

pub fn main() {
  echo case golden_value, Error(10.0, [100]) {
    9, Cv3([]) -> fn(v8) { 0.1 }(False)
    7, _ -> seed_value
    2, Cv4("abc", True) -> {
      {
        0.5
      } +. seed_value
    } -. {
      {
        let tag_value = 100.0
        10.0
      }
    }
    v9, _ -> case Cv1(42, 0.25), {
        let value = [4, 0]
        "x"
      } {
      v9, "abc" <> rest if rest == "x" && rest != "x" -> {
        let golden_value = 5
        let tag_value = True
        seed_value
      }
      Cv1(_, 0.0), "ab" -> 1.0
      v10, v11 -> 2.0
    }
  }
  echo "b"
}
