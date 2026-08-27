pub const golden_value: Bool = True
pub const tag_value: String = "a"
pub const euler_value: Float = 10.0

pub type V0 {
  None(value: String, inner: Int)
  Cv1
  Cv2(List(Int), Bool)
}

pub type V3 {
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(v5: V3, v6: List(Int), v7: Bool) -> Bool {
v7
}

fn export(acc: V0, delete: Float) -> List(Int) {
[0]
}

fn f2(s: String, value: V3, new: Int) -> Int {
4
}

pub fn main() {
  let length = {
    {
      let default = golden_value
      let item = [1]
      3.14
    }
  } /. {
    0.5
  }
  let euler_value = fn(v8) { 42 % 7 }(0.1)
  echo golden_value
  echo [42, 1]
}
