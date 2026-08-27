pub const pi_value: Float = 0.25

pub type Map {
  Record
  Cv0
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(default: Bool, v1: Bool) -> Bool {
default
}

pub fn main() {
  echo {
    let pi_value = {
      let y = {
        let new = 100
        let acc = "b"
        0.0
      }
      {
        let pi_value = "data"
        y
      }
    }
    let self_ = "x"
    [3]
  }
  echo "data" != "ab"
  echo pi_value
  echo "constructor"
}
