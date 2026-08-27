pub const pi_value: Bool = True

pub type Map {
  Record
}

pub type V0 {
  Cv1
  Cv2(value: Int, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int) -> Int {
{
    7 - 2
  } + {
    7 + spin(1, v3)
  }
}

fn delete(default: Int, v4: Bool, this_: V0) -> Float {
0.25
}

pub fn main() {
  let pi_value = {
    {
      let z = 42
      let pi_value = ""
      pi_value
    }
  } <> {
    fn(v5) { "res" }(2.0)
  }
  let self_ = 100
  echo self_
  echo [42]
}
