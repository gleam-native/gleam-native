pub const tag_value: String = "a"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, n: Int, v0: Bool) -> Int {
42
}

pub fn main() {
  echo {
    let m = 1 % 3
    let delete = {
      {
        let l = m
        let v = [4, 0]
        7
      }
    } + m
    []
  }
  echo 0.0
  echo case fn(v1) { [100, 3] }(1.0), 0 * 42 {
    [h], 1 if h <= 2 -> case {
        let tag_value = True
        let arguments = "bc"
        #(3.14, 10)
      } {
      #(2.0, _) | #(10.0, 8) -> 100 * 100
      inner -> fn(v2, v3) { h }("b", "b")
      #(10.0, 3) -> spin(7, h)
    }
    [b, _, ..], _ -> {
      b + 1
    } * 2
    _, v4 -> case tag_value <> "x" {
      "ab" <> item if item != "" -> 2
      inner | "abc" <> inner -> v4
      "bc" <> item -> fn(v5) { 100 }(True)
    }
  }
  echo case tag_value {
    inner -> case {
        let inner = True
        #("bc", True)
      }, [] {
      #(_, True), [9] -> [3]
      #("bc", False), [_, ..rest] -> fn(v6) { [5, 1] }("abc")
      v7, _ -> []
    }
    inner -> case 3.14 {
      item -> [5, 3]
      _ | 10.0 -> fn(v8) { [] }(3.14)
    }
  }
}
