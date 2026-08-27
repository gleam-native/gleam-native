pub const golden_value: String = "ab"
pub const limit_value: Float = 10.0
pub const tag_value: String = "ab"

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(n: Bool, v3: Int) -> Bool {
True
}

pub fn main() {
  let limit_value = [10, 4]
  echo case 10, tag_value {
    _, "bc" -> case 7 {
      _ -> !True
      _ | 7 -> True
    }
    9, _ -> False
    _, _ -> {
      {
        3.14
      } *. {
        100.0
      }
    } <=. {
      {
        0.5
      } -. {
        1.0
      }
    }
  }
  echo 4
  echo case [0] {
    [x] as whole -> case Cv2 {
      inner -> limit_value
      Cv2 -> []
      b -> {
        let l = True
        let value = limit_value
        []
      }
    }
    [constructor, 9, ..] as whole -> []
    _ -> []
  }
  echo True
}
