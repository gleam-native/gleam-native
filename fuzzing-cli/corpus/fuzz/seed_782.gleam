pub const seed_value: Float = 0.5
pub const tag_value: String = "ab"

pub type Symbol {
  Cv0(value: String, inner: Int)
  Number(value: Int, inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(v1: Symbol) -> List(Int) {
case "bc" <> "x" {
    acc -> case <<3:16, "ab":utf8>>, {
        let rest = [2, 100]
        let value = 0.25
        acc
      } {
      <<"x":utf8>>, _ -> fn(v2, v3) { [] }(False, "abc")
      <<"ab":utf8, "data":utf8>> as whole, "b" as it if it == "" || it == "a" -> [3, 0]
      _, acc -> [4, 100]
    }
    v4 -> []
    v5 -> []
  }
}

fn new(v6: Int, m: Int) -> Bool {
case {
      let self_ = [42, 0]
      m
    } {
    2 -> True
    _ -> case "x" <> "x", "a" {
      "data" as whole, "abc" if whole != "a" -> {
        let rest = 4
        True
      }
      "bc", "constructor" -> False
      _, v7 -> True
    }
    item -> {
      1.5
    } != {
      0.25
    }
  }
}

fn export(v8: Int, v9: #(List(Int), String), n: List(Int)) -> Bool {
{
    {
      1.0
    } +. {
      100.0
    }
  } >=. {
    case constructor(Number(2, 2.0)) {
      [8, ..rest] -> {
        let v9 = n
        1.0
      }
      [b, ..rest] -> 1.5
      v10 -> 1.0
    }
  }
}

pub fn main() {
  let n = case <<"a":utf8>> {
    <<"constructor":utf8>> -> fn(v11, v12) { "ab" }(10, "res")
    <<42:4, "bc":utf8, 10:4>> -> "bc"
    _ -> "x"
  }
  echo 10
  echo 3
  echo {
    case seed_value -. seed_value {
      item -> item
      item -> {
        0.5
      } -. item
      _ | 100.0 -> seed_value
    }
  } +. {
    0.25
  }
  echo case 7, 1 {
    5, 1 -> tag_value
    4, 0 -> fn(v13, v14) { fn(v15, v16) { n }(42, 0.5) }(False, False)
    _, v17 -> "bc"
  }
}
