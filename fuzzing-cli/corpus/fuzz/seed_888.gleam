pub const limit_value: String = "b"
pub const pi_value: Float = 3.14
pub const euler_value: Int = 1

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(m: Int) -> Bool {
False
}

pub fn main() {
  let y = False
  let y = {
    fn(v0, v1) { "ab" }(1, 0.0)
  } == "res"
  echo case euler_value + euler_value {
    _ -> [4]
    a -> fn(v2, v3) { {
      let default = limit_value
      let v2 = [5]
      v2
    } }(0.25, 0.0)
  }
  echo limit_value
  echo y
}
