pub const tag_value: String = "x"
pub const seed_value: Float = 2.0
pub const limit_value: Int = 4

pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Record(List(Int), value: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: Int, y: Float) -> Float {
{
    let length = case class, class + 4 {
      7, _ -> "b"
      9, _ -> "constructor"
      _, v3 -> "x" <> "res"
    }
    case Cv1([2, 2]) {
      constructor -> 0.5
      Record([5], class) if class <= 4 -> {
        let rest = []
        y
      }
      Cv2 -> {
        0.25
      } -. y
    }
  }
}

pub fn main() {
  let new = "a"
  let tag_value = {
    {
      10.0
    } *. seed_value
  } -. {
    10.0
  }
  echo fn(v4, v5) { v4 >= {
    10 |> spin(3)
  } }(2, False)
}
