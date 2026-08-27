pub const golden_value: Bool = True
pub const tag_value: Float = 10.0
pub const limit_value: Bool = False

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v: Int, item: #(String, String)) -> Int {
1
}

fn yield(v0: Int) -> List(Int) {
[3, 5]
}

pub fn main() {
  let self_ = case 0 - 0, "a" != "res" {
    5, True as whole if !whole || whole -> !True
    6, True -> 1 > 0
    _, True as whole -> True
    _, v1 -> golden_value
  }
  let self_ = case "ab" <> "x" {
    "ab" <> _ | "constructor" -> ""
    "data" <> b if b == "res" -> "data"
    "" <> _ -> "bc"
    _ -> {
      let length = 0.25
      "ab"
    }
  }
  echo False
  echo case self_, {
      let pair = []
      False
    } {
    _, _ -> self_
    "abc", l if !l && l -> self_
    golden_value, True -> case 3.14 {
      inner -> "x"
      0.0 | 10.0 -> fn(v2) { "abc" }("ab")
    }
  }
  echo [100, 1]
  echo f0(1, {
    {
      let z = True
      100
    }
  } - 100, {
    let l = fn(v3) { limit_value }(42)
    let acc = f0(100, 42, #("a", "b"))
    #("abc", "data")
  })
}
