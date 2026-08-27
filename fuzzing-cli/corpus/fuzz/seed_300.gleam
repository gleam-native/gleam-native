pub const golden_value: Float = 3.14

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn export(v2: List(Int)) -> List(Int) {
case fn(v3) { Cv1([4, 4]) }(3.14) {
    Cv1([v2, 3, ..]) -> [7]
    Cv1([]) -> v2
    _ -> case 2, [] {
      _, [] -> []
      prototype, [a, 1, ..] -> v2
      0, [0, ..rest] -> fn(v4) { rest }(7)
      _, _ -> v2
    }
  }
}

pub fn main() {
  let golden_value = golden_value
  echo 5
}
