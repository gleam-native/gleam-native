pub const seed_value: Int = 5
pub const limit_value: Bool = False
pub const golden_value: Int = 0

pub type V0 {
  Ok(value: String, inner: Float)
  Cv1(Bool, value: List(Int))
  Cv2
}

pub type Object {
  Some
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(delete: Int) -> String {
{
    let delete = case {
        let delete = delete
        Some
      } {
      Some | Some -> False
      Some -> {
        3.14
      } <=. {
        2.0
      }
    }
    "b" <> "abc"
  }
}

pub fn main() {
  let m = limit_value
  echo case {
      let s = golden_value
      let s = ""
      limit_value
    } {
    _ -> spin(fn(v3) { seed_value }("x"), spin(seed_value, 5))
    True | True -> {
      let limit_value = "ab"
      seed_value
    }
  }
  echo "data"
}
