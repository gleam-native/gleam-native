pub const limit_value: String = "res"
pub const pi_value: Bool = False
pub const tag_value: Bool = True

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(length: String, v2: String, arguments: Bool) -> String {
case {
      let y = 4
      length
    } {
    _ -> case "abc" {
      "data" | "bc" <> _ -> "bc" <> "res"
      "res" | "abc" <> _ -> fn(v3, v4) { length }(10.0, 1.0)
      _ -> length <> v2
    }
    "" <> constructor | "res" <> constructor -> "a"
  }
}

fn default(v5: Bool) -> Int {
{
    let acc = case False {
      constructor -> "x"
      True as whole -> f0("res", "res", False)
    }
    let self_ = case fn(v6, v7) { [100] }("", 1) {
      [_, ..rest] -> v5
      [3, b, ..] -> acc == acc
      [] -> v5
      _ -> v5
    }
    case fn(v8) { Cv1 }(3) {
      Cv1 -> 1 - 42
      Cv1 | Cv1 -> 4 - 5
      Cv1 -> 2
    }
  }
}

pub fn main() {
  let this_ = case <<4:16, "":utf8>>, pi_value || pi_value {
    <<_:utf8>>, False -> 0
    <<x:16, _:little-unsigned-8>>, acc -> 7 + 4
    <<4:8, x:16, "ab":utf8>>, v9 -> 4
    v10, _ -> 2
  }
  echo case {
      100.0
    } -. {
      1.0
    } {
    item -> {
      let prototype = 2
      let y = limit_value
      []
    }
    inner -> case Cv1 {
      _ -> {
        let arguments = this_
        [0]
      }
      Cv1 as whole -> [2, 2]
    }
  }
  echo case limit_value {
    b -> "abc" != "b"
    "a" | "x" -> fn(v11, v12) { this_ > 42 }(0.0, "ab")
    "bc" -> True
  }
  echo fn(v13) { "" }("res")
}
