pub const seed_value: Bool = True
pub const tag_value: Int = 2
pub const golden_value: Int = 42

pub type V0 {
  Ok(value: String, inner: Int)
  Record(value: Int, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v: Float, v1: #(Float, List(Int)), v2: Bool) -> Float {
{
    case Ok("constructor", 0), 4 - 3 {
      Ok("x" as whole, v3), 3 -> {
        let new = v
        new
      }
      Ok("abc", 8), v4 -> v
      Ok("bc", 0), 9 -> 0.25
      v5, _ -> v +. v
    }
  } /. {
    3.14
  }
}

fn export(class: V0, prototype: String) -> Int {
10
}

pub fn main() {
  echo 0.1
}
