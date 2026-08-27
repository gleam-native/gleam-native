pub const limit_value: Float = 1.0

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

fn static(class: #(String, Bool), length: Int) -> Int {
{
    42 % 2
  } - length
}

pub fn main() {
  let constructor = True
  echo case "bc" <> "constructor" {
    v3 -> {
      let z = [5, 2]
      fn(v4, v5) { v3 }(10, 10)
    }
    constructor | "bc" <> constructor -> case 7 {
      6 | 0 -> constructor
      4 | 3 -> constructor <> constructor
      v6 -> fn(v7) { constructor }(4)
    }
    inner | "bc" <> inner -> case Cv2, [100, 100] {
      Cv1, [_, 6, ..] -> {
        let z = []
        let z = inner
        "data"
      }
      Cv2, [] -> inner
      _, [_, _, ..] as whole -> "abc"
      _, v8 -> "b"
    }
  }
  echo {
    case "res" <> "ab" {
      v9 -> constructor
      "abc" -> True
      item -> constructor
    }
  } && {
    case Cv2, {
        let limit_value = True
        let limit_value = []
        0
      } {
      _, 6 -> False
      new, 3 -> True || constructor
      Cv2 as whole, 5 -> constructor || constructor
      _, v10 -> constructor
    }
  }
  echo case 42 {
    3 as whole if whole == 3 -> 1.0
    a -> {
      let x = 10
      fn(v11) { limit_value }("data")
    }
  }
  echo {
    5 - 4
  } + {
    {
      #("ab", True) |> static(walk([4, 3], 0))
    } * static(#("abc", False), 7)
  }
}
