pub const limit_value: String = "data"
pub const tag_value: Float = 1.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Some(Int)
  Cv2
}

pub type V3 {
  Cv4
  Cv5(Float, List(Int))
  Cv6(String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(z: Int, v7: Bool) -> String {
{
    case 10 * 0, 3 {
      z, 0 if z <= 0 -> "b" <> "res"
      1, 7 -> "b"
      _, 2 -> "constructor" <> "data"
      _, _ -> "x"
    }
  } <> {
    {
      let y = "a" <> "res"
      y
    }
  }
}

pub fn main() {
  let v = tag_value
  let tag_value = [5]
  echo v == {
    1.0
  }
}
