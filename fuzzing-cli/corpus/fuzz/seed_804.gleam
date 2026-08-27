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

fn f0(arguments: Bool, m: Float) -> Bool {
{
    let delete = [42]
    arguments
  }
}

pub fn main() {
  echo 3.14
  echo case fn(v3, v4) { #(42, "b") }("abc", "") {
    constructor -> True
    #(9, "res") -> case "ab" <> "bc" {
      "a" -> False
      b -> True
    }
  }
  echo {
    0.1
  } /. {
    1.0
  }
  echo spin(case Cv2 {
    class -> 3
    Cv1 | Cv1 -> 3
    Cv2 -> 42 - 5
  }, 10)
}
