pub const golden_value: Bool = False
pub const limit_value: String = "b"

pub type V0 {
  Error(value: String, inner: List(Int))
}

pub type Promise {
  Cv1(Int, value: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Int) -> Bool {
True
}

fn f1(v3: String, v: String) -> Int {
{
    let m = v
    let m = case v3 <> m {
      "abc" <> b -> True
      _ -> False
      "abc" -> f0(0)
    }
    case "bc" <> v {
      "abc" as whole -> spin(5, 4)
      constructor -> 4
    }
  }
}

pub fn main() {
  echo {
    case spin(5, 1) {
      inner -> 0.1
      constructor -> 10.0
      constructor -> {
        let this_ = 10
        let this_ = golden_value
        0.5
      }
    }
  } == {
    {
      {
        let arguments = 1.0
        let default = 2
        10.0
      }
    } *. {
      {
        3.14
      } *. {
        2.0
      }
    }
  }
}
