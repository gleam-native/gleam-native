pub const euler_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(arguments: List(Int)) -> Bool {
{
    let arguments = fn(v0) { fn(v1) { 1.0 }("a") }(True)
    True
  }
}

fn f1(v2: Bool, new: Int) -> String {
case "x" <> "" {
    "ab" -> "bc"
    _ -> case "a" {
      inner -> fn(v3, v4) { "" }(1, 1.5)
      _ -> {
        let new = False
        let l = 0
        "abc"
      }
    }
  }
}

pub fn main() {
  let euler_value = {
    "ab" <> "a"
  } <> "data"
  let z = [3] |> constructor()
  echo z
}
