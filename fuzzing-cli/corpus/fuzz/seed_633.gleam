pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Int, arguments: Int, m: Int) -> List(Int) {
[]
}

pub fn main() {
  echo {
    {
      let new = "abc" != "res"
      5 % 2
    }
  } - 2
  echo "x"
}
