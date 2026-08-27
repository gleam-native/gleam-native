pub const golden_value: Float = 100.0
pub const seed_value: Int = 3

pub type V0 {
  Number(value: String, inner: String)
  None
  Error(value: Bool, inner: Float)
}

pub type V1 {
  Cv2(value: List(Int), inner: Int)
  Cv3
  Cv4(value: Int, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(pair: Bool) -> List(Int) {
[]
}

pub fn main() {
  let this_ = fn(v5, v6) { 2 }(3, 0.0)
  let golden_value = False
  echo {
    let seed_value = "bc"
    let new = True |> arguments()
    case golden_value, {
        let acc = new
        new
      } {
      _, [0] as whole -> spin(10, this_)
      _, [x, a, ..] -> 3
      _, [] -> 0 |> spin(10 + this_)
      v7, _ -> 3
    }
  }
}
