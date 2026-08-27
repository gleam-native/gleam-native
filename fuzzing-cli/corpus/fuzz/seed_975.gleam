pub const euler_value: Float = 1.0
pub const golden_value: Bool = True
pub const seed_value: Int = 3

pub type Symbol {
  Record
  Cv0(Int)
}

pub type V1 {
  Cv2
  Cv3(Int, value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(value: String, v4: String) -> String {
value <> value
}

pub fn main() {
  let seed_value = "b"
  let golden_value = [2]
  echo golden_value
  echo walk(fn(v5, v6) { golden_value }(0.0, 4), 4)
  echo {
    {
      seed_value <> seed_value
    } <> "bc"
  } <> {
    case 1, Cv2 {
      y, rest -> f0(seed_value, "abc")
      3, Cv3(6, v7) if v7 % 2 == 0 -> fn(v8, v9) { seed_value }(True, True)
      2, _ -> fn(v10, v11) { "b" }(0, 1)
    }
  }
}
