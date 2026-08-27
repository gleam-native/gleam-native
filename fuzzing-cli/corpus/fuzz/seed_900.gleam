pub const limit_value: Float = 1.0
pub const pi_value: Bool = False

pub type V0 {
  Cv1
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int, constructor: Bool) -> String {
{
    let v3 = "bc"
    let new = 2
    v3
  }
}

fn export(value: Bool, v4: Float) -> String {
"ab"
}

pub fn main() {
  let this_ = case Cv1, 2 + 7 {
    Cv2, _ -> 0
    Cv2 as whole, 7 -> 3 |> spin(spin(2, 3))
    Cv2, 2 -> spin(3, 10)
    _, v5 -> 1
  }
  echo [3, 10]
  echo 0 - {
    this_ |> spin(this_ - 5)
  }
  echo case [2, 5], fn(v6) { this_ }(False) {
    [h, ..rest], _ if h == 1 -> case fn(v7) { h }(10) {
      a -> 100 >= 1
      _ | 4 -> pi_value
      _ -> False
    }
    [b], 1 -> case "x" {
      self_ -> False || pi_value
      "ab" -> True && True
      b | "b" <> b -> {
        let l = "ab"
        let value = l
        pi_value
      }
    }
    [7, b, ..], _ -> False
    _, _ -> True
  }
}
