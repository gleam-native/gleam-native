pub const seed_value: Bool = False
pub const golden_value: Int = 2

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: List(Int), new: List(Int), s: Int) -> Int {
spin({
    fn(v0) { s }("constructor")
  } + s, 2)
}

fn constructor(n: List(Int), v1: Int, arguments: String) -> String {
{
    let new = {
      let z = "bc"
      True
    }
    let z = {
      let m = "a"
      n
    }
    arguments
  }
}

pub fn main() {
  let z = case fn(v2, v3) { seed_value }(0.0, 0) {
    inner -> seed_value
    True -> False
    True -> {
      let n = [5]
      let length = golden_value
      seed_value
    }
  }
  echo []
  echo ""
  echo 0 - f0([], [], golden_value)
}
