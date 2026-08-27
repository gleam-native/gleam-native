pub const golden_value: Int = 5
pub const euler_value: Int = 0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(List(Int), Bool)
}

pub type V3 {
  Number(Bool)
  Cv4
}

pub type Number {
  Cv5
  Some
}

fn extends(v6: Int, new: Int, s: V0) -> Int {
case 0.25, "b" {
    _, _ -> 10
    1.5, "x" <> rest -> case Some {
      Some -> v6 - v6
      _ -> 3 + 7
    }
    3.14, _ -> 4
  }
}

fn arguments(v7: Bool, v8: Float, v9: Bool) -> Bool {
v7
}

pub fn main() {
  let item = case {
      let l = 3
      let m = golden_value
      100.0
    } {
    100.0 | 0.1 -> [7]
    b -> fn(v10, v11) { [] }(7, False)
  }
  let default = case 0.1, "data" <> "" {
    1.5, "abc" -> item
    _, _ -> item
  }
  echo 0.5
  echo {
    case "res" <> "ab" {
      "x" <> _ | "ab" -> "a"
      _ -> "x"
      inner -> "b"
    }
  } <> {
    {
      let delete = fn(v12) { 1 }(10)
      let acc = fn(v13, v14) { v13 }(1, 3.14)
      "data"
    }
  }
  echo fn(v15, v16) { extends(golden_value, 42 * euler_value, Cv1([])) }(1.5, 0.1)
}
