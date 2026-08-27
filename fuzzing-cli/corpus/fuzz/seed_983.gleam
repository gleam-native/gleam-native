pub const golden_value: Bool = False
pub const limit_value: Bool = False
pub const seed_value: Bool = True

pub type Record {
  Record
}

pub type V0 {
  Cv1(value: Bool)
  Cv2(value: Int)
  Cv3
}

pub type V4 {
  Cv5
  Cv6
  Cv7(Bool)
}

fn f0(new: String, v8: Float) -> Bool {
case fn(v9) { "" }(1.0), "x" <> "" {
    "bc", "ab" -> case [2] {
      [8, constructor, ..] -> 2 != constructor
      [constructor, ..rest] as whole -> True
      [_] -> "ab" != "bc"
      _ -> True
    }
    _, v10 -> fn(v11) { True }("bc")
    "constructor", "abc" <> _ -> True
  }
}

pub fn main() {
  let new = {
    let length = fn(v12) { seed_value }(3)
    let acc = !length
    f0("data", 0.5)
  }
  echo {
    {
      {
        2.0
      } -. {
        0.0
      }
    } +. {
      {
        2.0
      } +. {
        10.0
      }
    }
  } +. {
    case #(7, False), [42, 2] {
      #(_, True), [] -> 3.14
      #(1, _), [5, ..rest] -> {
        0.0
      } *. {
        100.0
      }
      #(0, _), [] as whole -> {
        0.1
      } -. {
        2.0
      }
      _, _ -> fn(v13, v14) { 100.0 }(False, False)
    }
  }
}
