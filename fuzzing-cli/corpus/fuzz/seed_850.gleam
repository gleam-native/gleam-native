fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(n: Int) -> Int {
100
}

pub fn main() {
  let z = case {
      let item = [3, 10]
      let acc = 10
      acc
    }, {
      let v = True
      let s = True
      [1]
    } {
    9, [2] -> []
    item, [9] if item > 1 && item <= 7 -> [7]
    _, [a, ..rest] -> rest
    v0, _ -> {
      let n = [5, 1]
      let v = v0
      n
    }
  }
  let length = 42
  echo case length % 4, {
      let constructor = 2.0
      7
    } {
    v1, 0 if v1 > 9 -> fn(v2) { fn(v3, v4) { "constructor" }(2, "constructor") }(5)
    4, v -> {
      "abc" <> ""
    } <> "x"
    v5, _ -> case {
        0.5
      } +. {
        10.0
      } {
      0.0 -> "bc"
      constructor -> {
        let length = 0.25
        "ab"
      }
      n -> "bc"
    }
  }
  echo {
    let length = case "data" <> "data", 3 {
      _, 2 -> z
      "a" <> _, _ -> z
      "ab" <> rest, 7 -> z
      v6, v7 -> z
    }
    {
      let z = constructor(42)
      length
    }
  }
  echo case True, length {
    True, 8 -> True
    v8, _ -> v8
    length, class -> length
  }
  echo {
    {
      fn(v9) { 0.5 }(0.5)
    } -. {
      {
        0.1
      } -. {
        0.25
      }
    }
  } == {
    0.1
  }
}
