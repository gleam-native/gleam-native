pub const limit_value: String = "constructor"
pub const seed_value: Bool = True

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

fn arguments(v3: Int, acc: V0) -> List(Int) {
case fn(v4) { Cv2 }(True) {
    Cv2 -> fn(v5) { {
      let l = 0.25
      let l = 1
      [4]
    } }(3)
    Cv2 -> case [] {
      [3] -> fn(v6) { [3] }(True)
      [9, ..rest] -> fn(v7) { rest }(False)
      [6] -> [10, 5]
      _ -> []
    }
    _ -> case "abc" {
      item -> [42]
      v8 -> [1, 4]
    }
  }
}

pub fn main() {
  echo case "res" <> limit_value {
    b -> {
      {
        1.5
      } -. {
        100.0
      }
    } -. {
      0.25
    }
    "b" | "res" <> _ -> {
      let limit_value = fn(v9, v10) { [] }(42, 0.5)
      let s = "abc"
      {
        let delete = 0.5
        let rest = s
        delete
      }
    }
  }
  echo {
    {
      {
        let limit_value = 1
        let self_ = "data"
        self_
      }
    } <> {
      "b" <> limit_value
    }
  } <> {
    case fn(v11) { True }(100.0) {
      item -> limit_value <> limit_value
      True | True -> fn(v12, v13) { "" }(2, 5)
    }
  }
}
