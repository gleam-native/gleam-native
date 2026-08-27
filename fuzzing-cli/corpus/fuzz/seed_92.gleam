pub const golden_value: Bool = True
pub const tag_value: Int = 7
pub const euler_value: Int = 4

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v3: Int) -> Bool {
False
}

pub fn main() {
  let rest = golden_value
  echo case fn(v4) { Cv2(0.5) }(7) {
    _ -> {
      let new = "data" <> "bc"
      let new = False
      {
        let y = 42
        [0]
      }
    }
    inner -> case 2 {
      4 -> {
        let inner = "bc"
        [1]
      }
      b -> []
      8 -> [2, 7]
    }
    constructor -> case "" <> "b" {
      "a" <> rest | "ab" <> rest -> fn(v5) { [] }(1.0)
      constructor -> [100]
      constructor -> [3, 10]
    }
  }
  echo {
    3.14
  } *. {
    case True {
      False -> 0.0
      a -> 2.0
      _ -> {
        1.0
      } -. {
        2.0
      }
    }
  }
  echo [7]
  echo 1.5
}
