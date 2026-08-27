pub const tag_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(new: Float) -> Float {
new
}

pub fn main() {
  let n = case "abc", {
      0.1
    } <. {
      0.25
    } {
    "x" <> _, True -> "a"
    "res" <> rest, _ -> rest <> ""
    _, _ -> "res"
  }
  echo [1]
  echo [7, 7]
}
