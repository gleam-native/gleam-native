fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(prototype: Int) -> List(Int) {
case "", "ab" {
    "abc", _ -> case #(0, [5, 3]) {
      b -> [1, 4]
      #(6, [1, _, ..]) as whole -> []
    }
    "ab" <> rest, "x" -> case rest {
      _ -> [0, 0]
      "res" -> []
    }
    v0, v1 -> [42, 4]
  }
}

fn f1(v2: Int, item: Float, pair: Int) -> Float {
case [4, 2] {
    [4, ..rest] -> 100.0
    [b] as whole -> 0.0
    [_, 2, ..] -> case 1.0 {
      _ -> item *. item
      _ | 10.0 -> 1.0
    }
    _ -> fn(v3) { item }(5)
  }
}

fn export(rest: String) -> String {
rest
}

pub fn main() {
  let pair = walk([3], 0)
  echo "abc"
  echo fn(v4) { case 5 - 0, <<"x":utf8>> {
    default, <<_:utf8>> if default > 9 && default <= 1 -> 0 |> constructor()
    3, <<_:utf8>> -> constructor(100)
    2 as whole, _ -> []
    _, _ -> constructor(pair)
  } }(False)
  echo [0]
  echo {
    fn(v5) { 100 }("a")
  } - 10
}
