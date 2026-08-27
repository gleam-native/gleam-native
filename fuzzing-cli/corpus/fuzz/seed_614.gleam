pub const seed_value: Float = 0.1
pub const golden_value: Float = 1.5

pub type V0 {
  Ok(value: String, inner: Bool)
}

pub type V1 {
  Cv2
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(rest: Int, prototype: String, x: String) -> Float {
{
    1.5
  } *. {
    0.0
  }
}

fn f1(v4: #(List(Int), Bool), l: Int) -> List(Int) {
case l {
    5 -> []
    a -> []
  }
}

pub fn main() {
  let s = 3
  echo {
    fn(v5, v6) { s - 5 }(True, 7)
  } |> spin(100)
  echo [5, 4]
}
