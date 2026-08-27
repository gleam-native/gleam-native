pub type Promise {
  Cv0(value: String, inner: List(Int))
  Cv1(value: Float)
}

pub type V2 {
  Cv3(Bool)
}

pub type V4 {
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v6: Int) -> Bool {
True
}

fn constructor(arguments: Float, v7: Bool, v8: #(Int, String)) -> String {
"bc"
}

fn f2(v9: Promise, class: String, v10: List(Int)) -> String {
"bc" <> class
}

pub fn main() {
  echo 3
  echo case 2 {
    x -> f0(10 * 10)
    constructor -> True
  }
  echo walk({
    let new = 100 + 4
    fn(v11, v12) { [] }(0, 1.5)
  }, walk([5], 2) * 1)
  echo False
}
