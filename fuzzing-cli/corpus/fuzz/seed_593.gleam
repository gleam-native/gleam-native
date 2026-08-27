pub const limit_value: Bool = False
pub const euler_value: Bool = False

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type Promise {
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Float, v: Int, new: #(Float, List(Int))) -> List(Int) {
[]
}

fn f1(l: String, delete: V0) -> Bool {
True || {
    case {
        2.0
      } -. {
        0.0
      }, Cv1([10], 3) {
      _, Cv1([], v4) if v4 % 2 == 0 || v4 == 1 -> True
      v5, _ -> 3 == 1
      _, Cv1([5], 0) -> False
    }
  }
}

fn f2(s: String, v6: Bool, y: #(String, Float)) -> Float {
1.0
}

pub fn main() {
  let euler_value = case fn(v7) { 7 }(1) {
    b -> "a" <> "bc"
    constructor -> ""
    5 -> "constructor"
  }
  echo 100.0
}
