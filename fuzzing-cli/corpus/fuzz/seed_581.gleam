pub const seed_value: String = "res"
pub const tag_value: Int = 1
pub const pi_value: String = "ab"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  None(Int, Float)
  Cv2(Float, value: Int)
}

pub type V3 {
  Record(value: String, inner: Bool)
  Error
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(new: V0, v4: Int) -> String {
fn(v5) { {
    "x" <> "constructor"
  } <> "res" }(True)
}

fn f1(v6: Int) -> String {
yield(Cv1([10, 42], 1), spin(spin(v6, 10), spin(v6, v6)))
}

pub fn main() {
  let v = fn(v7, v8) { pi_value }(False, 1.5)
  echo fn(v9) { case {
      let y = 0.5
      let v9 = 0.0
      Record("a", True)
    } {
    Error -> fn(v10, v11) { [0] }("a", 4)
    Error | Error -> []
    _ -> []
  } }(1)
  echo {
    case [] {
      [_, ..rest] -> v <> "a"
      [2, ..rest] -> {
        let v = []
        let value = seed_value
        "x"
      }
      [0, a, ..] -> pi_value <> seed_value
      _ -> pi_value
    }
  } <> {
    "b" <> "res"
  }
  echo [42, 1]
}
