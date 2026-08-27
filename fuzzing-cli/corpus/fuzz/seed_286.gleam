pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(pair: String) -> List(Int) {
[2]
}

pub fn main() {
  let v = case "", True {
    "ab" <> rest as whole, v3 -> rest <> "res"
    "b", True -> "a"
    v4, _ -> "abc"
  }
  echo 1
  echo {
    0.5
  } *. {
    0.0
  }
}
