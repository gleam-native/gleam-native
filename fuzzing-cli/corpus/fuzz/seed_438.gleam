pub const seed_value: String = "a"
pub const pi_value: String = "constructor"

pub type V0 {
  None(value: String, inner: Float)
  Cv1
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(x: List(Int), m: String) -> String {
case "res" {
    "a" -> m
    inner -> "ab"
    _ -> {
      {
        let l = [1]
        m
      }
    } <> {
      fn(v2) { "bc" }(2.0)
    }
  }
}

fn f1(x: Float, class: Int) -> Int {
42 + {
    case Cv1 {
      None("res" <> rest, 2.0) if rest != "constructor" -> 100 |> spin(spin(class, class))
      Cv1 as whole -> spin(3, 10)
      v3 -> spin(100, class)
    }
  }
}

pub fn main() {
  echo {
    let default = {
      0.25
    } +. {
      0.1
    }
    {
      True && False
    } || {
      False || True
    }
  }
}
