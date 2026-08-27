pub type V0 {
  Error(value: String, inner: List(Int))
  Ok(value: Bool, inner: List(Int))
  Some(value: List(Int), inner: Float)
}

pub type V1 {
  Cv2(value: Bool, inner: Float)
  Cv3(value: Float, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(length: Bool, item: List(Int), v4: String) -> Int {
2
}

fn f1(x: Bool) -> Float {
{
    0.1
  } -. {
    {
      let x = fn(v5) { [0] }(7)
      1.0
    }
  }
}

fn f2(v6: Float, prototype: Bool) -> String {
fn(v7) { fn(v8) { v7 <> v7 }(False) }("ab")
}

pub fn main() {
  let y = 4
  echo [0]
  echo case 0, [5, 1] {
    8, [_, ..rest] -> rest
    9, [_, 3, ..] as whole -> whole
    v9, [1] as whole -> fn(v10) { fn(v11) { [3] }(1.5) }(7)
    _, _ -> []
  }
  echo []
  echo y
}
