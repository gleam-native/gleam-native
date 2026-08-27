pub const pi_value: String = "abc"
pub const limit_value: String = "bc"
pub const golden_value: String = ""

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(new: Float, pair: V0) -> String {
"res"
}

pub fn main() {
  let rest = arguments({
    let this_ = []
    let acc = 1.5
    acc
  }, {
    let golden_value = 0.25
    let x = 1
    Cv1([42])
  })
  let constructor = case arguments(3.14, Cv1([7, 3])) {
    "b" as whole -> True
    _ -> {
      let limit_value = []
      let value = limit_value
      True
    }
  }
  echo 4 * {
    case {
        let m = "bc"
        let delete = 5
        Cv1([])
      }, {
        let item = 2
        let prototype = []
        Cv1([])
      } {
      Cv1([9, _, ..]), _ -> 1
      Cv1([]), Cv1([] as whole) -> fn(v2, v3) { 10 }("", "bc")
      v4, v5 -> 3 * 1
    }
  }
  echo constructor
}
