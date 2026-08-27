pub const seed_value: Float = 100.0
pub const euler_value: Int = 4
pub const tag_value: String = "ab"

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3
  Cv4(Float, value: Bool)
  Cv5(List(Int), Bool)
}

pub type V6 {
  Cv7(value: Int, inner: Bool)
  Cv8(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(z: Float, new: Bool) -> List(Int) {
case "b" <> "constructor", {
      let length = [3]
      7
    } {
    "data" <> rest, 4 if rest == "ab" -> [0]
    "constructor" <> rest as whole, 6 -> []
    v9, _ -> [4]
  }
}

pub fn main() {
  let tag_value = case Cv5([10], True) {
    _ | Cv5(_, _) -> {
      let rest = "data"
      let seed_value = euler_value
      [0]
    }
    _ -> [3]
    b -> {
      let y = False
      let y = "constructor"
      [10, 7]
    }
  }
  let seed_value = case Cv7(7, True) {
    _ -> [0]
    inner -> tag_value
  }
  echo {
    let z = fn(v10, v11) { f0(v11, v10) }(False, 1.5)
    case {
        0.5
      } -. {
        0.1
      } {
      inner -> {
        let z = 2.0
        let euler_value = euler_value
        tag_value
      }
      b -> {
        let s = b
        seed_value
      }
      3.14 -> tag_value
    }
  }
  echo case "abc" {
    "ab" as whole -> 0.0
    "bc" -> 0.0
    _ -> 0.0
  }
  echo euler_value
}
