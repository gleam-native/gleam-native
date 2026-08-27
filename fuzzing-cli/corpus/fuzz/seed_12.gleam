pub const golden_value: Float = 3.14

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Cv3(value: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(v4: String) -> Bool {
True
}

pub fn main() {
  echo {
    case "x", Cv3(42) {
      "ab", Cv3(3) as whole -> fn(v5, v6) { golden_value }(10, 0.5)
      "bc" <> rest, Cv3(8) as whole -> golden_value +. golden_value
      _, _ -> golden_value
    }
  } -. {
    golden_value +. {
      golden_value /. {
        2.0
      }
    }
  }
  echo {
    {
      0.1
    } -. {
      {
        let length = []
        0.5
      }
    }
  } +. {
    case fn(v7) { Cv1([], 42) }(1.5), 4 {
      Cv1([_, 5, ..], 8) as whole, 0 -> 2.0
      Cv1([9, ..rest], prototype), _ -> 2.0
      _, _ -> {
        let y = 1
        let v = []
        2.0
      }
    }
  }
}
