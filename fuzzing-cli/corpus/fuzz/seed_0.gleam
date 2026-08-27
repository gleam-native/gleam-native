pub const euler_value: Float = 1.0
pub const seed_value: String = "a"

pub type Object {
  Record
  Cv0(value: Float)
  Cv1(Bool, Int)
}

pub type Number {
  Cv2(value: String, inner: Float)
  Cv3
}

fn extends(s: #(Float, List(Int))) -> String {
case {
      let class = True
      let z = 0.1
      4
    } {
    constructor -> "b"
    0 -> {
      "b" <> "x"
    } <> "res"
  }
}

pub fn main() {
  let value = "ab" <> "x"
  echo {
    {
      euler_value -. {
        0.5
      }
    } *. {
      {
        let constructor = False
        let value = [5]
        euler_value
      }
    }
  } /. {
    0.5
  }
  echo 0.5
  echo {
    "data" <> "a"
  } <> {
    {
      let item = seed_value
      "abc"
    }
  }
  echo extends(fn(v4, v5) { {
    let pair = v4
    let constructor = 3.14
    #(0.0, [])
  } }(True, "abc"))
}
