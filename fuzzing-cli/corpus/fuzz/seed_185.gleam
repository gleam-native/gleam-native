pub const golden_value: Int = 7

pub type Promise {
  Cv0(value: String, inner: String)
  Cv1(Bool)
}

pub type V2 {
  Cv3(value: Bool, inner: List(Int))
}

pub type V4 {
  Record(Float)
  Cv5(List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(v6: Int) -> Int {
case 5 {
    6 -> 0 - {
      {
        let v6 = 100.0
        let v6 = "b"
        100
      }
    }
    b -> 7
  }
}

fn constructor(delete: Float, v7: #(String, Float), default: Int) -> List(Int) {
case "ab" {
    "b" | "x" <> _ -> []
    delete -> []
  }
}

pub fn main() {
  let y = 7
  echo False
  echo [5, 5]
  echo False
  echo 1.0
}
