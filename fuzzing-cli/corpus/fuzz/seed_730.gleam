pub type Symbol {
  Cv0(value: String, inner: Float)
  Ok(Bool, Int)
  Cv1(Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Int) -> Float {
{
    0.0
  } -. {
    {
      {
        0.5
      } -. {
        2.0
      }
    } +. {
      0.25
    }
  }
}

pub fn main() {
  let item = case f0(7) {
    _ | 0.0 -> {
      0.1
    } *. {
      0.0
    }
    constructor -> constructor
  }
  let item = "ab"
  echo 5
}
