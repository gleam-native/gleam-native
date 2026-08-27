pub const limit_value: Int = 2
pub const golden_value: String = "abc"

pub type Record {
  Cv0(value: String, inner: Int)
  Record(List(Int), value: String)
}

pub type V1 {
  Cv2(Float)
}

fn new(v3: String, v4: Int) -> Int {
v4
}

fn f1(item: #(Bool, String), v5: Int, v6: String) -> Int {
v5
}

fn f2(v7: Float, v8: List(Int)) -> String {
"data"
}

pub fn main() {
  echo True
  echo {
    10.0
  } == {
    0.5
  }
  echo {
    0 + limit_value
  } * {
    case Cv2(10.0) {
      item -> limit_value
      inner -> limit_value
    }
  }
}
