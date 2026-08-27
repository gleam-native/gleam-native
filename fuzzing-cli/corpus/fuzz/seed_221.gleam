pub const limit_value: Int = 1

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(item: V0) -> Int {
10 * 0
}

fn f1(pair: #(Float, Float), l: String) -> Int {
{
    case Cv2, l {
      Cv2, _ -> 100
      Cv2, "a" -> 100
      _, "data" -> f0(Cv2)
      v3, v4 -> 4
    }
  } - {
    case fn(v5, v6) { "" }(True, 2.0) {
      item -> 7
      item -> {
        let pair = 0
        pair
      }
      "b" | "abc" -> 4
    }
  }
}

fn f2(default: V0) -> Bool {
case False || True, {
      0.0
    } +. {
      1.0
    } {
    True, 3.14 -> fn(v7, v8) { v8 }(3, True)
    pair, v9 -> pair
    rest, 0.0 -> rest
  }
}

pub fn main() {
  let self_ = {
    10.0
  } /. {
    1.0
  }
  echo {
    {
      "bc" <> "bc"
    } <> {
      fn(v10, v11) { "bc" }(True, "a")
    }
  } <> "constructor"
  echo case limit_value % 6, True {
    5, _ -> {
      fn(v12) { 1.0 }("x")
    } == {
      self_ *. {
        3.14
      }
    }
    9, v13 -> case #(2.0, [10]), 1.0 {
      #(1.0, [a, ..rest]), _ -> v13
      #(0.1 as whole, [_, _, ..]), 1.0 -> self_ >=. self_
      _, v14 -> v13
    }
    v15, v16 -> {
      let v15 = v15
      False
    }
  }
}
