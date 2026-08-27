fn new(constructor: Bool, v0: Bool, v1: #(List(Int), Int)) -> List(Int) {
case {
      let rest = [5, 100]
      "bc"
    } {
    "constructor" <> rest -> [100, 2]
    _ | "res" -> [10]
    v1 -> [100, 42]
  }
}

pub fn main() {
  let m = 3
  let item = case [3], "" {
    [_], _ -> [42]
    [_], "x" -> new(False, True, #([5, 100], 4))
    v2, _ -> [0, 42]
  }
  echo case m * 7 {
    5 -> case True, #(0.25, 7) {
      False, #(10.0 as whole, 8) -> "res" <> "bc"
      True, #(2.0, 9 as whole) as it -> "data" <> "constructor"
      _, _ -> "res" <> "constructor"
    }
    v3 -> case True |> new(True, #([], 42)) {
      [_, 4, ..] -> "x" <> "res"
      [x] if x % 2 == 0 || x > 2 -> {
        let length = 5
        "data"
      }
      [9, 2, ..] -> {
        let prototype = ""
        let m = 10
        prototype
      }
      _ -> "bc" <> "constructor"
    }
    _ -> case "data" {
      "ab" as whole -> fn(v4) { "abc" }(0.25)
      "b" -> fn(v5) { v5 }("bc")
      "data" <> rest -> "constructor" <> "x"
      v6 -> fn(v7, v8) { v6 }(False, 100.0)
    }
  }
  echo item
  echo case m {
    _ -> m
    item -> {
      fn(v9, v10) { 7 }(True, True)
    } + item
    4 -> m
  }
  echo 0.25
}
