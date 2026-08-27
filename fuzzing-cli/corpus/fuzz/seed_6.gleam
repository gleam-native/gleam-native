pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(pair: Int, item: Int, v2: Int) -> Int {
case {
      let item = [100, 1]
      let acc = [7, 3]
      #(0.25, [])
    } {
    a -> {
      pair - item
    } - {
      fn(v3, v4) { 5 }(3, 0.5)
    }
    #(v5, []) -> fn(v6, v7) { v2 }(0.1, 2)
  }
}

fn f1(y: List(Int), arguments: Float, v: Float) -> Bool {
case fn(v8, v9) { [42, 10] }("data", 100.0), {
      let y = v
      "res"
    } {
    [8], "x" <> rest as whole -> case 10 {
      a -> True
      5 -> {
        let self_ = True
        let v = 1
        False
      }
      inner -> True
    }
    [b, _, ..], v10 if v10 != "bc" -> 1 != class(b, b, b)
    [5, ..rest], "bc" <> _ as whole -> False
    _, v11 -> True
  }
}

pub fn main() {
  let this_ = 42
  echo 1.5
  echo True
}
