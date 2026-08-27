pub const golden_value: Bool = True

pub type V0 {
  Record(value: String, inner: Bool)
  Cv1(List(Int))
  Cv2(List(Int), Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(v: Bool, v3: List(Int)) -> String {
{
    let v = case [] {
      [7] -> True
      [h] -> {
        0.5
      } <=. {
        0.5
      }
      v4 -> False
    }
    let arguments = {
      let item = 4 + 1
      "a" <> "data"
    }
    case 100 - 1, 0 {
      v5, 4 if v5 == 9 -> arguments
      0, 1 -> "abc"
      v6, v7 -> arguments <> arguments
    }
  }
}

pub fn main() {
  echo fn(v8) { case [], default(golden_value, []) {
    [7, h, ..], _ -> {
      0.25
    } +. {
      0.0
    }
    [b], "constructor" -> 2.0
    _, v9 -> fn(v10, v11) { 0.5 }(True, 0)
  } }(3)
}
