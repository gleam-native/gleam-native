pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(v2: Int) -> List(Int) {
[]
}

fn f1(class: #(Int, String), v3: V0, x: V0) -> List(Int) {
[10]
}

pub fn main() {
  let l = True
  let l = case 100 * 4 {
    b -> [100]
    a -> []
    constructor -> []
  }
  echo 2.0
  echo l
  echo case 4 * 42 {
    2 -> 1.0
    b -> {
      0.5
    } +. {
      1.0
    }
  }
  echo "constructor"
}
