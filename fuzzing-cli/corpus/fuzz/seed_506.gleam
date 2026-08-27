pub const golden_value: Bool = True
pub const euler_value: String = ""

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(n: Bool) -> String {
{
    case n, {
        let arguments = 0.5
        let class = [1]
        class
      } {
      _, [_] -> {
        let class = [1]
        "abc"
      }
      True, [6, ..rest] -> "b"
      _, _ -> "bc" <> "data"
    }
  } <> "a"
}

pub fn main() {
  echo []
  echo [1, 5]
}
