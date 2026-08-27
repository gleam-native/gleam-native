pub const tag_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, v0: Int, value: Float) -> Float {
constructor
}

fn yield(y: Int) -> List(Int) {
[7, 42]
}

pub fn main() {
  let this_ = yield(walk([3, 100], 4))
  echo 2.0
  echo {
    let self_ = case 10 |> yield(), fn(v1) { #("abc", 4) }(True) {
      [_, x, ..], #("a" <> rest, _) as whole if rest == "b" || rest != "" -> {
        let self_ = x
        let tag_value = []
        False
      }
      [], #(v, 2) -> 1 <= 0
      v2, _ -> {
        let class = 10
        True
      }
    }
    let z = fn(v3, v4) { "ab" }(10, 7)
    case this_ {
      [constructor, ..rest] if constructor <= 6 -> "bc" <> "ab"
      [] -> z
      v5 -> z <> z
    }
  }
}
