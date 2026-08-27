pub const limit_value: Float = 2.0
pub const euler_value: Int = 7
pub const tag_value: Int = 7

pub type Symbol {
  Record
  Cv0(Bool)
  Number
}

pub type V1 {
  Cv2(Float, String)
  Cv3(Int, value: Int)
}

fn f0(n: String) -> List(Int) {
case {
      let prototype = 1.0
      n
    }, {
      let z = 1
      n
    } {
    "a" as whole, v4 if v4 == "a" && whole != "res" -> [2]
    v5, "bc" <> rest if rest != "" -> [100]
    "res", z -> [0, 3]
    _, _ -> case 10 - 2 {
      b -> [0, 100]
      0 -> fn(v6, v7) { [] }(0, "res")
    }
  }
}

fn extends(v8: Int, v9: Int) -> Int {
v9
}

pub fn main() {
  let value = 10.0
  echo {
    case True, 4 {
      _, new -> 2
      True, _ -> extends(100, 100)
      True, 8 -> tag_value + euler_value
    }
  } * {
    case 3.14 {
      a -> 3 + tag_value
      _ -> extends(euler_value, euler_value)
      _ | 0.5 -> {
        let l = True
        let m = euler_value
        tag_value
      }
    }
  }
}
