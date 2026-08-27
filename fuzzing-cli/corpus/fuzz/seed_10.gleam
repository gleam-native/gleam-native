pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Cv3(value: String, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(this_: V0, pair: Bool, v4: V0) -> Int {
{
    let new = case "ab" <> "a" {
      "a" <> rest -> rest <> "res"
      "constructor" | "abc" <> _ -> "x" <> "bc"
      _ -> "constructor"
    }
    let self_ = case spin(100, 3), v4 {
      v5, _ -> [10]
      3 as whole, Cv2 -> [5, 3]
      7, x -> [42]
    }
    0
  }
}

pub fn main() {
  let n = {
    let this_ = "x"
    this_
  }
  echo []
  echo case Cv2 {
    Cv3("bc" <> rest, 6) if rest == "constructor" && rest == "res" -> "res" <> rest
    Cv2 -> "ab" <> "bc"
    _ -> n
  }
}
