pub const seed_value: Bool = True
pub const tag_value: Int = 100
pub const pi_value: Float = 0.1

pub type Promise {
  Cv0(value: String, inner: Bool)
  Cv1(Float, Int)
}

pub type Map {
  Cv2
  Cv3
}

pub type V4 {
  Number
}

fn f0(v5: Bool, arguments: Int, n: #(String, Bool)) -> String {
{
    case 100 - 1, Cv0("bc", True) {
      9, Cv1(3.14 as whole, _) as it -> fn(v6) { v6 }("constructor")
      8, Cv1(0.0, 5 as whole) -> "x" <> "constructor"
      _, _ -> "abc" <> ""
    }
  } <> "x"
}

pub fn main() {
  echo [5, 5]
  echo {
    case "x" {
      "data" | "bc" -> {
        let acc = True
        pi_value
      }
      v7 -> 0.0
    }
  } +. {
    {
      let value = {
        let new = 10.0
        []
      }
      0.1
    }
  }
  echo seed_value
  echo fn(v8) { v8 }(1)
}
