pub const golden_value: Bool = False
pub const pi_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: Int, rest: List(Int), pair: Bool) -> String {
"res"
}

fn static(item: String) -> Float {
fn(v3) { case item <> item {
    "res" <> inner | "ab" <> inner -> {
      0.5
    } +. {
      2.0
    }
    item -> {
      3.14
    } -. {
      0.0
    }
  } }(False)
}

pub fn main() {
  let golden_value = fn(v4) { True }(42)
  echo 1.5
  echo case 4 {
    inner -> inner
    8 -> fn(v5, v6) { 7 }(0.25, True)
  }
}
