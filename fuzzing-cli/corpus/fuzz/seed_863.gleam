pub const seed_value: String = ""
pub const limit_value: String = "x"

pub type V0 {
  Ok(value: String, inner: Bool)
  Cv1(value: String)
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(y: Int, prototype: Float, class: String) -> String {
{
    "a" <> {
      {
        let y = [4]
        let y = 7
        class
      }
    }
  } <> {
    "a" <> class
  }
}

fn default(v3: String, n: String) -> Bool {
False
}

pub fn main() {
  echo []
}
