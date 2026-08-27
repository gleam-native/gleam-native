pub const golden_value: Bool = False
pub const euler_value: Int = 1

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Bool)
  Cv3(value: String, inner: Int)
}

fn static(v4: #(Float, List(Int)), n: #(Bool, List(Int))) -> Bool {
False
}

pub fn main() {
  let default = {
    {
      1.0
    } +. {
      0.25
    }
  } *. {
    3.14
  }
  let class = static({
    let l = [10]
    #(2.0, [])
  }, #(False, [100, 3]))
  echo {
    2.0
  } <=. {
    {
      let y = []
      let v = 0.5
      0.0
    }
  }
  echo case "b" <> "b" {
    "abc" | "constructor" -> case {
        let x = euler_value
        let new = "res"
        new
      }, "res" {
      "constructor", "data" <> rest if rest == "data" -> rest <> "ab"
      _, "" <> rest -> {
        let default = 0
        let default = rest
        rest
      }
      _, _ -> "x" <> "b"
    }
    a -> "abc" <> {
      {
        let golden_value = [10, 0]
        let euler_value = a
        a
      }
    }
  }
  echo {
    euler_value % 1
  } != {
    euler_value + 42
  }
}
