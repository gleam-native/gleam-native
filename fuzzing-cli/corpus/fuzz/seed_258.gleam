pub const limit_value: Int = 3
pub const euler_value: Float = 0.1
pub const golden_value: Bool = False

pub type Record {
  Cv0(value: String, inner: Int)
  Ok(value: String)
  Some
}

fn f0(acc: Bool, x: Bool) -> Int {
7
}

pub fn main() {
  echo {
    {
      fn(v1) { euler_value }(True)
    } -. {
      {
        let limit_value = "bc"
        euler_value
      }
    }
  } /. {
    0.5
  }
  echo True
  echo !{
    case #(1, "x"), "x" {
      #(_, _) as whole, "constructor" <> rest -> golden_value
      #(6 as whole, v2), length -> golden_value
      #(v3, "abc"), v4 -> limit_value == 10
      v5, _ -> False
    }
  }
}
