pub const limit_value: Int = 100

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(class: Bool) -> Int {
case "ab" {
    "constructor" <> _ | "res" -> {
      {
        let rest = []
        let rest = 100
        rest
      }
    } - {
      fn(v0) { 10 }("x")
    }
    "b" -> 10
    _ -> case {
        let class = "abc"
        5
      } {
      _ | 6 -> {
        let x = []
        let class = "abc"
        2
      }
      inner -> {
        let x = 1.0
        let m = inner
        m
      }
    }
  }
}

fn f1(item: Float) -> Float {
case {
      let acc = 1
      True
    } {
    False | False -> case 0 + 100 {
      6 | 3 -> {
        let z = item
        let y = True
        z
      }
      8 | 4 -> item
      _ -> fn(v1, v2) { 100.0 }(True, 3)
    }
    constructor -> {
      {
        let item = [1, 7]
        let acc = item
        100.0
      }
    } -. {
      {
        let z = "b"
        let l = [7]
        0.5
      }
    }
    z -> {
      {
        let rest = 10.0
        item
      }
    } *. {
      {
        1.0
      } -. item
    }
  }
}

pub fn main() {
  let limit_value = case "a" <> "x", fn(v3, v4) { [5] }(2, False) {
    _, [] -> {
      let limit_value = limit_value
      limit_value
    }
    "ab", [] -> limit_value * 100
    "data" <> rest, [3] -> limit_value |> spin(5)
    _, v5 -> 0 |> spin(2 - limit_value)
  }
  let new = True
  echo {
    {
      2.0
    } == {
      0.1
    }
  } && {
    case <<"x":utf8>> {
      <<"a":utf8>> -> False
      <<"a":utf8>> -> True
      _ -> new
    }
  }
}
