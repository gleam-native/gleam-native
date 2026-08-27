pub const tag_value: Bool = True

pub type V0 {
  Number(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(self_: V0, v1: Bool) -> Float {
0.5
}

fn default(x: List(Int), v: #(Int, Float)) -> String {
case "data", "bc" <> "x" {
    "ab", "constructor" -> case walk(x, 7), x {
      6, [a] -> "data" <> "constructor"
      v2, [_] -> "x" <> "abc"
      _, _ -> "b"
    }
    "bc", "data" -> case 3 + 10 {
      v3 -> ""
      0 -> "res"
      4 | 9 -> {
        let n = 0.25
        let n = 100
        "constructor"
      }
    }
    "res", "b" <> rest -> case 5 * 3 {
      9 | 6 -> rest <> rest
      v4 -> "abc"
      _ -> "res" <> rest
    }
    _, _ -> "abc"
  }
}

fn static(prototype: Float, constructor: Int, this_: Int) -> Float {
{
    f0(Number("x", 2.0), False) -. {
      100.0
    }
  } +. {
    {
      let n = 0.5
      let prototype = []
      0.25
    }
  }
}

pub fn main() {
  let new = {
    0.25
  } +. {
    {
      1.0
    } *. {
      3.14
    }
  }
  echo [10]
  echo {
    let l = False
    let length = {
      let tag_value = []
      let new = {
        let item = 3.14
        0
      }
      fn(v5, v6) { tag_value }(100.0, "")
    }
    fn(v7, v8) { [7] |> default(#(3, 100.0)) }("", True)
  }
}
