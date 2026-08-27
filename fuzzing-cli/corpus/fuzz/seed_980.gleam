pub const seed_value: Float = 10.0

pub type V0 {
  Cv1
  Cv2
  Cv3(String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(constructor: Int, v4: V0, v5: Int) -> String {
"" <> "a"
}

fn f1(value: Int) -> Int {
spin(case [] {
    [b] if b <= 3 -> b * 7
    [h] -> spin(h, 4)
    _ -> value
  }, {
    let x = [3]
    let arguments = fn(v6, v7) { [0, 100] }(0.25, 5)
    value
  })
}

pub fn main() {
  echo case "bc" {
    item | "constructor" <> item -> case 3 {
      2 -> "b"
      4 -> "a" <> "bc"
      _ -> 1 |> static(Cv2, fn(v8) { 5 }(0.25))
    }
    "abc" -> fn(v9, v10) { "b" }("constructor", True)
  }
  echo fn(v11) { v11 }(False)
  echo [2, 10]
}
