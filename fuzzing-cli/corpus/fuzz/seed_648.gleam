pub const golden_value: Float = 0.1

pub type V0 {
  Error(value: String, inner: Int)
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(v2: String, n: List(Int)) -> String {
v2
}

pub fn main() {
  echo "res"
  echo False || False
  echo {
    let rest = 10
    let rest = case "res" <> "bc" {
      b -> fn(v3, v4) { [7, 2] }(2, True)
      item -> fn(v5) { [100, 7] }("ab")
    }
    case "res" {
      "bc" <> rest -> [3, 3]
      "ab" <> _ -> [1]
      _ -> rest
    }
  }
}
