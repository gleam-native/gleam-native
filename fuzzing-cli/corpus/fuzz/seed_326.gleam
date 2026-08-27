fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, value: Bool, this_: Bool) -> List(Int) {
[1]
}

pub fn main() {
  let l = {
    "ab" <> "constructor"
  } <> "x"
  echo True
  echo f0(False, True, case 10 - 100, False {
    v0, _ -> fn(v1) { True }(42)
    4, False -> False
    _, v2 -> v2
  })
}
