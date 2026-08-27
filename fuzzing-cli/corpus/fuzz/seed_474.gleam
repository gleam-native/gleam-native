pub const golden_value: Float = 0.25
pub const limit_value: String = "x"
pub const euler_value: String = "a"

pub type V0 {
  Cv1
  Cv2
  Cv3(value: List(Int))
}

fn f0(v4: String, v5: List(Int), v6: String) -> Int {
{
    {
      fn(v7) { 2 }("bc")
    } % 3
  } + {
    {
      fn(v8) { v8 }(42)
    } - 7
  }
}

pub fn main() {
  let s = "x"
  let euler_value = case fn(v9) { limit_value }(2.0), "x" <> s {
    "abc" <> rest, "a" -> 3
    _, _ -> 42
  }
  echo case fn(v10, v11) { Cv1 }("constructor", ""), s <> "x" {
    Cv3([x]), "bc" -> case 2, 10.0 {
      _, 3.14 -> fn(v12) { 1.0 }(0.25)
      euler_value, 10.0 -> golden_value -. {
        0.25
      }
      v13, v14 -> {
        1.0
      } *. {
        1.0
      }
    }
    Cv1, "ab" as whole -> {
      {
        let arguments = True
        golden_value
      }
    } -. {
      {
        1.0
      } +. golden_value
    }
    v15, v16 -> 0.1
  }
  echo {
    let golden_value = f0(s, [4], s) + {
      1 * euler_value
    }
    True
  }
  echo {
    euler_value - {
      euler_value + euler_value
    }
  } * euler_value
}
