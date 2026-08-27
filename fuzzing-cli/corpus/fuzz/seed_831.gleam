pub const euler_value: String = "abc"
pub const pi_value: Int = 1

pub type Number {
  Record
  Cv0
}

pub type V1 {
  Cv2
  Cv3
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(l: Int) -> Bool {
True
}

pub fn main() {
  let prototype = euler_value
  echo {
    let prototype = {
      prototype <> prototype
    } <> euler_value
    "abc"
  }
  echo f0(pi_value)
  echo []
  echo True
}
