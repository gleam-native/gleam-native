pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Record(value: Int, inner: String)
}

pub type Promise {
  Cv2
  Cv3
}

fn delete(v4: #(Float, Int)) -> Bool {
case #("res", 5) {
    inner -> case "constructor" <> "data" {
      _ -> False
      b -> "abc" != "b"
    }
    #("res" <> rest, 8) -> case rest {
      "res" -> {
        let self_ = 100.0
        True
      }
      "constructor" <> _ | "a" <> _ -> True
      v5 -> "a" != "b"
    }
    #(_, 2) | #("bc" <> _, 6) -> False
  }
}

fn f1(value: Promise, n: Float, z: String) -> Bool {
case fn(v6) { 4 }(True), [7] {
    2, [7, 0, ..] -> delete(#(1.0, 0)) || False
    _, [5] -> delete(#(1.5, 1))
    v7, [_, _, ..] -> case "abc" {
      "b" -> "ab" == z
      "abc" | "constructor" -> True && True
      "ab" -> delete(#(1.0, 7))
      _ -> delete(#(0.1, 5))
    }
    v8, _ -> case v8 {
      1 -> v8 != 3
      8 | 0 -> True
      9 -> True
      _ -> "" == z
    }
  }
}

fn f2(m: Promise, v9: Int) -> Float {
{
    3.14
  } -. {
    case False {
      y -> {
        3.14
      } /. {
        10.0
      }
      True | False -> 0.5
      constructor -> {
        3.14
      } +. {
        1.0
      }
    }
  }
}

pub fn main() {
  echo case {
      3.14
    } +. {
      3.14
    } {
    100.0 | 1.5 -> {
      2 * 5
    } - 42
    _ | 0.1 -> 0
    0.5 | 0.1 -> {
      0 - 3
    } * 0
  }
  echo case {
      let rest = 5
      []
    }, 4 {
    [] as whole, 1 as it -> True
    [4, _, ..], 2 -> True
    [7], 7 -> {
      let pair = 10.0
      let pair = False || True
      False
    }
    v10, v11 -> True
  }
  echo {
    fn(v12) { Cv3 }(4)
  } |> f2(7 + 100)
  echo False
}
