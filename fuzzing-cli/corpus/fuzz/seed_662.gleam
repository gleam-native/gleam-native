pub const euler_value: Bool = True

pub type V0 {
  Number(value: String, inner: String)
}

pub type Map {
  Ok(List(Int))
  Cv1
  Cv2(value: Float, inner: Float)
}

pub type V3 {
  Cv4
  Cv5(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v6: String, default: Float) -> Float {
default
}

pub fn main() {
  let length = "data"
  let euler_value = "ab" <> "x"
  echo case True {
    length -> case {
        let arguments = 2
        euler_value
      } {
      inner | "ab" <> inner -> 1.0
      constructor -> 10.0
    }
    a -> 0.5
    True as whole -> f0(10, "res" <> length, 1.0)
  }
  echo True
}
