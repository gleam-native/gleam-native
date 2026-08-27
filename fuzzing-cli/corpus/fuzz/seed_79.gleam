pub const golden_value: String = "data"

pub type Number {
  Record
  Cv0
  Number(List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(m: Int) -> List(Int) {
[]
}

pub fn main() {
  echo fn(v1) { {
    let y = v1
    {
      let constructor = 2
      let rest = constructor
      golden_value
    }
  } }(True)
}
