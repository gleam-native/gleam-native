pub const euler_value: Bool = True
pub const pi_value: Bool = True
pub const limit_value: Float = 3.14

pub type Promise {
  Cv0(value: String, inner: Int)
  Error(List(Int))
}

pub type V1 {
  Cv2(Int, value: Float)
  Cv3(value: Int)
  Cv4
}

fn static(delete: #(String, String)) -> Float {
case <<"x":utf8>> {
    <<_:utf8, "res":utf8>> -> case 10 * 42 {
      3 -> {
        1.5
      } +. {
        1.0
      }
      item -> {
        0.25
      } -. {
        0.5
      }
      rest -> 0.1
    }
    <<_:utf8>> as whole -> 1.0
    _ -> 3.14
  }
}

fn default(v5: Int, z: String) -> Bool {
True
}

pub fn main() {
  echo static(case pi_value, {
      let pi_value = []
      let n = limit_value
      #(True, 2.0)
    } {
    False, #(_, _) -> #("bc", "a")
    _, #(_, 100.0) -> #("abc", "x")
    _, v6 -> fn(v7, v8) { #("", "abc") }(True, "b")
  })
  echo 0
}
