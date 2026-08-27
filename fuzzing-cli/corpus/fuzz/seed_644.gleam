pub const euler_value: Float = 10.0

pub type V0 {
  Some(value: String, inner: Int)
  Cv1(Float)
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(v3: List(Int), y: Bool) -> List(Int) {
case "res" {
    "data" <> rest -> case fn(v4) { v4 }("abc") {
      "" <> _ -> v3
      "" <> rest -> [42]
      v5 -> {
        let acc = 42
        let arguments = acc
        v3
      }
    }
    _ | "b" -> [1, 42]
    "res" -> [1]
  }
}

pub fn main() {
  echo class([5], False)
}
