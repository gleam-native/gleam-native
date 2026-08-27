pub const pi_value: Bool = True
pub const tag_value: String = "a"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(class: String) -> Bool {
True || {
    fn(v0) { fn(v1, v2) { False }("b", 10) }(True)
  }
}

pub fn main() {
  let class = case [5, 10] {
    [a, ..rest] as whole -> True
    [b] -> True
    _ -> constructor(tag_value)
  }
  echo fn(v3) { case 5 {
    6 -> pi_value || pi_value
    v4 -> v3
  } }(False)
  echo tag_value
  echo case spin(10, 4) {
    8 as whole if whole > 2 -> {
      {
        let x = tag_value
        let class = 2.0
        whole
      }
    } - spin(whole, 42)
    v5 -> 2
    inner -> inner
  }
}
