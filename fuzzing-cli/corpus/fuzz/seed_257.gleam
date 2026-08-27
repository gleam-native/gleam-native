pub const limit_value: String = "bc"
pub const tag_value: String = "b"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(x: Float, v3: String, length: Int) -> String {
{
    let m = length == length
    {
      let y = 0.5
      let value = ""
      v3 <> ""
    }
  }
}

pub fn main() {
  let limit_value = case 4 % 7 {
    9 as whole if whole > 3 -> True
    _ -> True
    1 | 0 -> {
      let tag_value = 2.0
      let s = 1
      True
    }
  }
  let y = 2
  echo 3.14
}
