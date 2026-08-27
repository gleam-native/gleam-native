pub const seed_value: Bool = False

pub type V0 {
  Some(value: String, inner: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(v1: Bool, prototype: Float) -> Int {
{
    fn(v2) { 5 |> spin(7 * 7) }("a")
  } + 100
}

fn arguments(delete: String, v3: Int) -> Float {
{
    let v3 = [100, 4]
    let s = {
      {
        0.5
      } *. {
        1.5
      }
    } -. {
      0.1
    }
    case 2.0, v3 {
      10.0, [3, ..rest] -> 2.0
      0.5, [b] if b > 8 -> s /. {
        3.14
      }
      10.0, [s, 1, ..] -> {
        1.5
      } *. {
        0.0
      }
      v4, v5 -> v4 -. {
        2.0
      }
    }
  }
}

pub fn main() {
  let default = 4
  echo case {
      let l = 2.0
      Some("a", "")
    } {
    constructor -> 3.14
    Some("ab", "data") | Some(_, _) -> case Some("res", "ab"), fn(v6, v7) { v7 }(True, "res") {
      Some(seed_value, "constructor" <> rest as whole) as it, "abc" if whole == "ab" || whole != "constructor" -> fn(v8) { v8 }(100.0)
      Some("x", "ab" as whole), _ -> 0.25
      Some("bc", _), "b" -> {
        let v = "ab"
        3.14
      }
      _, v9 -> {
        let item = False
        let value = [2]
        100.0
      }
    }
  }
  echo {
    let self_ = case Some("a", "ab"), default {
      Some("data" <> rest, _), 4 -> new(True, 0.0)
      Some("abc", "res"), 7 -> 1 - default
      Some("res" <> rest, length), 5 -> {
        let default = seed_value
        let rest = 0.25
        42
      }
      v10, _ -> 0 + default
    }
    let default = case 10, fn(v11) { [] }(True) {
      2, [_] -> [3]
      6, [] -> []
      7, [] -> []
      _, _ -> {
        let self_ = 1.5
        let x = default
        [4, 3]
      }
    }
    {
      {
        let l = "x"
        let class = default
        l
      }
    } <> {
      {
        let v = 1.0
        let this_ = seed_value
        "b"
      }
    }
  }
  echo {
    let new = default
    let pair = "b"
    0 - 100
  }
}
