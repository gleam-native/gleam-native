pub type V0 {
  Ok(value: String, inner: Float)
  Cv1(value: Bool)
}

pub type Promise {
  Cv2(value: Int)
  Cv3(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(v4: #(List(Int), List(Int))) -> Int {
{
    let v4 = spin(100, 100) <= 1
    let new = case Cv2(2) {
      Cv3(1) as whole -> []
      _ -> [42]
    }
    4 * 0
  }
}

fn f1(y: Int) -> String {
case "ab" <> "res", y {
    "constructor" as whole, v5 -> whole
    _, y -> "b"
  }
}

pub fn main() {
  echo 10 + {
    42 + spin(0, 42)
  }
  echo {
    f1(4) <> "x"
  } <> "data"
  echo case {
      let constructor = 42
      Cv1(True)
    }, {
      0.5
    } *. {
      0.1
    } {
    _, 0.1 -> True
    _, 2.0 -> True
    v6, _ -> case {
        10.0
      } != {
        1.0
      } {
      True | True -> {
        3.14
      } == {
        1.0
      }
      True as whole if whole || whole -> whole
      True -> True
      v7 -> {
        0.1
      } <=. {
        0.5
      }
    }
  }
}
