pub const limit_value: Bool = False

pub type V0 {
  None(value: String, inner: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(length: List(Int)) -> Bool {
True
}

fn f1(v1: Bool, v2: String) -> Float {
3.14
}

fn extends(acc: Int, v: Float, v3: Int) -> List(Int) {
[]
}

pub fn main() {
  let item = case [] |> walk(2), "bc" <> "a" {
    _, "constructor" <> rest if rest != "" && rest == "a" -> rest <> "b"
    _, "res" <> rest -> "abc"
    _, v4 -> v4
  }
  let item = True
  echo case limit_value {
    True -> extends(100, 3.14, 2)
    False as whole -> [3, 10]
    _ -> []
  }
}
