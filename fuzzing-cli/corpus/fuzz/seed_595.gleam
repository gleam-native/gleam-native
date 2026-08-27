pub const pi_value: Float = 1.5
pub const tag_value: Bool = False
pub const euler_value: Int = 3

pub type Record {
  Cv0(value: String, inner: List(Int))
  Cv1
  Cv2(value: Float)
}

fn f0(rest: Int, delete: #(List(Int), Bool)) -> Int {
0 - {
    {
      rest + rest
    } % 7
  }
}

pub fn main() {
  echo case f0(euler_value, #([10], True)) {
    1 -> case fn(v3, v4) { "bc" }(42, True) {
      item | "" <> item -> {
        let m = tag_value
        let pi_value = tag_value
        ""
      }
      item -> item
      _ -> ""
    }
    8 | 0 -> "a" <> {
      {
        let l = 1.5
        let value = 0.1
        "res"
      }
    }
    inner -> ""
  }
}
