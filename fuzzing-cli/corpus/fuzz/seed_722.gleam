pub const limit_value: Int = 3
pub const seed_value: Int = 0
pub const euler_value: String = "constructor"

pub type Record {
  Cv0(value: String, inner: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v1: Bool, v2: Bool) -> Int {
spin(10 % 5, 4)
}

fn f1(v3: Bool) -> Int {
7
}

fn f2(n: Float) -> String {
{
    case "abc" <> "x" {
      _ -> "x"
      "a" <> a -> "constructor"
      "bc" -> {
        let n = [4]
        ""
      }
    }
  } <> {
    {
      let n = n
      let m = 2 - 5
      "a"
    }
  }
}

pub fn main() {
  echo [5]
}
