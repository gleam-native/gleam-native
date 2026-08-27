pub const euler_value: String = "abc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: Int, v1: Bool) -> Int {
walk(case v0 {
    5 -> {
      let n = 2.0
      let z = [3]
      [2, 10]
    }
    3 as whole if whole > 2 -> []
    _ -> [1, 4]
  }, v0)
}

pub fn main() {
  echo euler_value
}
