pub const tag_value: Bool = True
pub const euler_value: Bool = False

fn f0(m: String, y: List(Int), new: List(Int)) -> Float {
{
    {
      {
        1.5
      } -. {
        0.1
      }
    } +. {
      fn(v0, v1) { 100.0 }(True, "bc")
    }
  } +. {
    {
      let v = 7 == 7
      let m = fn(v2) { 10 }(0.1)
      fn(v3, v4) { 10.0 }("b", False)
    }
  }
}

fn yield(constructor: Int, s: Bool) -> Bool {
100 != {
    case s || False {
      v5 -> constructor + 0
      True as whole if !whole -> constructor % 5
      False | False -> 0
    }
  }
}

fn new(v6: Int, constructor: Bool) -> Int {
{
    case 2 {
      constructor -> v6 * v6
      7 | 4 -> fn(v7, v8) { v6 }("abc", True)
      0 -> fn(v9) { 100 }(1)
    }
  } + 5
}

pub fn main() {
  echo case "x", #(False, 42) {
    "" <> rest, #(True, 6 as whole) if rest != "x" -> new(4, True)
    "b" <> rest, #(z, _) as whole -> new(3, 100 < 4)
    _, _ -> case 10.0, 2 {
      v10, v11 -> new(4, False)
      _, 4 -> 4
      0.1, 5 -> 3 * 7
    }
  }
  echo tag_value
  echo {
    4 |> new(euler_value)
  } - {
    {
      let new = {
        let rest = tag_value
        10
      }
      let v = fn(v12, v13) { v12 }(0, False)
      v - new
    }
  }
  echo {
    let tag_value = "bc"
    let tag_value = "b" <> tag_value
    euler_value
  }
}
