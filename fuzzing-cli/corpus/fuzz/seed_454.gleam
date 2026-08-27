pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: List(Int))
  Cv3(value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(item: Int, pair: String) -> String {
case #([], True) {
    #([0], False) -> case pair <> "constructor" {
      _ -> {
        let pair = True
        "b"
      }
      "a" -> "constructor" <> pair
      "ab" as whole -> "res"
    }
    #([constructor, _, ..], item) if constructor <= 0 -> "data"
    #([item], True) -> "x"
    v4 -> {
      "a" <> pair
    } <> pair
  }
}

fn f1(v5: Int) -> Float {
2.0
}

fn new(v6: String, v7: Int, self_: List(Int)) -> Int {
v7
}

pub fn main() {
  let item = "ab"
  echo case fn(v8, v9) { Cv2([0, 3]) }(10.0, False) {
    Cv2([4]) | Cv3(_) -> []
    _ -> [3]
    Cv3("bc") as whole -> [0, 1]
  }
  echo {
    {
      {
        0.5
      } -. {
        10.0
      }
    } -. {
      10.0
    }
  } != {
    fn(v10) { 2.0 }("res")
  }
}
