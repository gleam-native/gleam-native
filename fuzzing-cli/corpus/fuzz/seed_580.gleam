pub const golden_value: Int = 1

pub type Map {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(z: Map) -> Float {
0.1
}

pub fn main() {
  let s = case fn(v0, v1) { Record }(False, 0.0) {
    Record -> False
    item -> True
  }
  echo case #("abc", [1]) {
    #("b", []) -> case "abc" <> "b", [] {
      constructor, [x, _, ..] if constructor == "constructor" || constructor == "" -> [1]
      _, [1, ..rest] -> rest
      _, v2 -> fn(v3) { v2 }(False)
    }
    #("a", []) -> [42]
    #("a" <> rest, [6, ..tail]) -> {
      let rest = {
        1.5
      } /. {
        0.5
      }
      let class = [10, 4]
      fn(v4, v5) { tail }("constructor", 0.1)
    }
    _ -> [10]
  }
}
