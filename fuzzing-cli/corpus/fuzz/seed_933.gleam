pub const euler_value: Float = 2.0
pub const seed_value: Float = 3.14
pub const pi_value: Int = 4

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type Record {
  Cv3(Int)
  None
  Ok(Int, Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(l: #(Float, List(Int)), v4: Int, acc: Int) -> List(Int) {
[3]
}

pub fn main() {
  let length = {
    fn(v5) { euler_value }("bc")
  } +. {
    {
      10.0
    } *. seed_value
  }
  echo {
    fn(v6) { 3 }(False)
  } - {
    pi_value % 6
  }
}
