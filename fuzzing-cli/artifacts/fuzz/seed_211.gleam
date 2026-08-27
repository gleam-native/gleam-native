pub type V0 {
  Number(value: String, inner: Float)
  Cv1
}

pub type V2 {
  Cv3(List(Int))
  Error
}

pub type V4 {
  Cv5(Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(prototype: #(Bool, Int), v6: #(Int, Float)) -> Bool {
False
}

fn extends(m: List(Int), pair: List(Int)) -> String {
"a" <> {
    case <<1:8>> {
      <<_:utf8>> -> "ab" <> "ab"
      _ -> "b"
    }
  }
}

fn f2(m: Int, v7: Bool) -> Int {
m
}

pub fn main() {
  let n = case "bc" {
    item -> {
      let default = 2.0
      let default = 0.1
      False
    }
    _ | "constructor" -> constructor(#(True, 4), #(4, 100.0))
  }
  let x = !n
  echo {
    case 5 {
      b -> b - 42
      a -> a
      0 -> 1
    }
  } |> f2(constructor(#(True, 2), #(0, 0.5)))
  echo case <<"b":utf8>> {
    <<_:utf8>> -> 0.0
    <<"data":utf8>> -> 1.5
    _ -> 2.0
  }
  echo case 0.25, Number("", 1.5) {
    3.14, Number("ab" <> rest, 0.1) -> 3.14
    0.5 as whole, Cv1 -> whole
    0.1, _ -> case [2] {
      [] -> {
        1.0
      } /. {
        2.0
      }
      [h] -> {
        let m = h
        100.0
      }
      _ -> {
        let n = 1
        let x = ""
        0.0
      }
    }
    v8, _ -> {
      fn(v9, v10) { 0.5 }(0.1, False)
    } *. {
      10.0
    }
  }
  echo case True, extends([1], [42]) {
    False as whole, "bc" if whole && !whole -> {
      {
        let whole = 0
        0.25
      }
    } +. {
      fn(v11) { 0.5 }(True)
    }
    True, "res" -> fn(v12) { {
      10.0
    } -. {
      1.0
    } }("")
    v13, _ -> {
      {
        100.0
      } -. {
        0.1
      }
    } +. {
      {
        0.0
      } +. {
        1.0
      }
    }
  }
}
