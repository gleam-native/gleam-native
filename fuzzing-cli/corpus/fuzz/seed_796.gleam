pub const golden_value: Int = 42
pub const tag_value: Int = 0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Ok(List(Int))
}

pub type Number {
  Cv3(List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(rest: List(Int), delete: List(Int), arguments: List(Int)) -> String {
""
}

fn f1(s: String) -> Float {
{
    {
      1.0
    } -. {
      0.0
    }
  } +. {
    0.5
  }
}

fn f2(v4: #(Bool, Float), delete: Float) -> Int {
fn(v5) { 2 - 4 }(0.25)
}

pub fn main() {
  echo 42
  echo {
    fn(v6) { fn(v7, v8) { "abc" }(42, True) }(True)
  } <> {
    fn(v9, v10) { "b" <> "b" }(False, 3)
  }
  echo 3
}
