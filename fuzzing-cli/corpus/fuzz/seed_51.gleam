pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Cv4
}

pub type Number {
  Cv5
  Number
  None(value: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(v6: Int) -> String {
"b" <> "x"
}

fn f1(delete: String, y: Bool) -> List(Int) {
[42, 4]
}

fn yield(m: Int) -> List(Int) {
[10]
}

pub fn main() {
  let value = {
    let pair = [10]
    "b"
  }
  echo [5]
  echo case 3 * 4 {
    _ -> {
      let value = {
        0.0
      } >. {
        10.0
      }
      let rest = {
        0.1
      } +. {
        0.0
      }
      {
        let value = ""
        False
      }
    }
    8 -> case 4 - 4 {
      inner -> True
      _ -> False && True
      prototype -> True && False
    }
  }
}
