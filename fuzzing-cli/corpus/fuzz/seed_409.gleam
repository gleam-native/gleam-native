pub const euler_value: String = "ab"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(z: Bool) -> Bool {
z
}

pub fn main() {
  let euler_value = {
    fn(v0) { 0.5 }(4)
  } /. {
    3.14
  }
  echo fn(v1, v2) { euler_value *. {
    fn(v3) { 100.0 }(4)
  } }(2.0, True)
}
