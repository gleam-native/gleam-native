pub const tag_value: Int = 3

pub type Record {
  Cv0(value: String, inner: Float)
  Cv1(value: Bool, inner: Int)
  Cv2(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(n: List(Int)) -> Bool {
case 3.14 {
    item -> {
      let n = fn(v3, v4) { v4 }(4, 4)
      let n = "a"
      False
    }
    _ -> False
  }
}

fn f1(v5: Record, this_: Int) -> Int {
this_
}

fn f2(this_: Int, v6: Bool, v7: Float) -> Bool {
v6
}

pub fn main() {
  let pair = {
    fn(v8, v9) { v9 }(True, "x")
  } <> {
    "x" <> "abc"
  }
  echo case pair <> "abc" {
    _ | "abc" -> case {
        let tag_value = False
        pair
      } {
      _ | "a" <> _ -> f1(Cv0("a", 0.25), tag_value)
      b -> tag_value + 42
      _ -> 2 * tag_value
    }
    b -> 1
  }
  echo False
  echo {
    case pair {
      v10 -> Cv1(True, 10)
      a -> fn(v11) { Cv2("res") }(False)
    }
  } |> f1(Cv2("ab") |> f1(fn(v12) { tag_value }(0.5)))
}
