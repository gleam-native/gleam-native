pub const pi_value: Bool = False
pub const golden_value: Int = 3

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: V0, v3: Bool) -> Float {
1.0
}

fn f1(v: Int, v4: Bool, v5: String) -> Bool {
v4
}

pub fn main() {
  let pi_value = case pi_value {
    a -> [3]
    inner -> [3, 1]
  }
  echo walk(case Cv2, {
      let pair = golden_value
      let pair = "bc"
      "abc"
    } {
    Cv2 as whole, "constructor" -> []
    Cv1, "ab" -> pi_value
    _, _ -> pi_value
  }, {
    {
      let prototype = True
      let prototype = golden_value
      golden_value
    }
  } - {
    {
      let z = [100, 4]
      golden_value
    }
  })
  echo True
}
