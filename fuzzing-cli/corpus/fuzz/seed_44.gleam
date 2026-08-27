pub const golden_value: Float = 0.0

pub type Number {
  Record
  Cv0(value: String)
  Cv1(List(Int), value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v2: Int, class: String) -> List(Int) {
[]
}

pub fn main() {
  echo case "" {
    "a" <> _ as whole if whole == "" || whole == "constructor" -> {
      let golden_value = whole
      {
        0.25
      } +. {
        2.0
      }
    }
    "ab" <> inner -> {
      golden_value -. {
        10.0
      }
    } +. {
      0.0
    }
    _ -> case [2, 1] {
      [8, ..rest] -> golden_value
      [] -> 10.0
      _ -> fn(v3) { golden_value }(True)
    }
  }
  echo True
}
