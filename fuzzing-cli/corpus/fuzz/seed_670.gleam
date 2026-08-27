pub const pi_value: Int = 10
pub const seed_value: Float = 10.0

pub type V0 {
  Some(value: String, inner: Bool)
}

pub type V1 {
  Error(value: String, inner: String)
  Cv2
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(n: Int, pair: String, item: Int) -> Int {
{
    {
      7 * item
    } * walk([], item)
  } * {
    fn(v4) { fn(v5, v6) { n }(10.0, "b") }(False)
  }
}

fn yield(v7: V0) -> Int {
case "" {
    constructor -> 3
    a | "abc" <> a -> fn(v8, v9) { [3, 100] |> walk(f0(5, a, 100)) }(3.14, False)
    a -> 0
  }
}

fn f2(v10: Float, v11: Int) -> Bool {
case {
      let constructor = []
      let value = 0
      #([3, 100], 42)
    }, {
      let class = "x"
      let v10 = "data"
      v11
    } {
    #([2], v12), 8 if v12 % 2 == 0 -> case Some("constructor", True) {
      Some("b", v13) if !v13 && v13 -> True
      Some("data" <> _, _) as whole -> "b" == "b"
      v14 -> True
    }
    #([0, ..rest], 5), 7 -> v10 == v10
    _, _ -> case 2 - v11, False {
      prototype, v15 -> True
      _, True -> False
      v, False -> True || False
    }
  }
}

pub fn main() {
  let pi_value = fn(v16) { pi_value }(0)
  let seed_value = case "bc" <> "x", #(True, False) {
    "a" <> rest, #(_, _) -> []
    "data" <> _, #(False, _) -> {
      let pi_value = True
      let prototype = "bc"
      [1]
    }
    _, v17 -> fn(v18) { [42, 5] }(True)
  }
  echo [42]
  echo {
    case {
        0.5
      } -. {
        0.1
      } {
      0.25 -> pi_value
      _ -> {
        let pi_value = 42
        let y = pi_value
        pi_value
      }
      length -> pi_value
    }
  } != pi_value
}
