pub const pi_value: Int = 100
pub const tag_value: Float = 0.1

pub type V0 {
  Record(value: String, inner: Float)
  Cv1(String)
  Ok(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(x: V0) -> Float {
{
    1.5
  } +. {
    0.25
  }
}

pub fn main() {
  echo {
    1.5
  } -. {
    case fn(v2, v3) { pi_value }(1.5, 0.1) {
      7 | 5 -> 0.25
      _ -> tag_value /. {
        2.0
      }
    }
  }
}
