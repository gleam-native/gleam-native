fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, y: Bool, class: Bool) -> Int {
constructor % 1
}

pub fn main() {
  echo ""
  echo case fn(v0, v1) { "ab" }(0.5, 4) {
    constructor -> True
    constructor -> True
    a -> False
  }
  echo case #(0, [0]) {
    #(9 as whole, [9]) as it -> case fn(v2, v3) { [0] }(True, 0.25) {
      [h] -> fn(v4, v5) { [4] }(False, False)
      [3, ..rest] as whole -> []
      [_] -> [2]
      v6 -> v6
    }
    #(3, [_]) as whole -> case fn(v7, v8) { v8 }("", 2) {
      5 -> [2, 4]
      6 -> [10]
      v9 -> {
        let n = 100.0
        [42, 10]
      }
    }
    #(2, [1, _, ..]) -> [3]
    v10 -> [4]
  }
  echo [5, 3]
}
