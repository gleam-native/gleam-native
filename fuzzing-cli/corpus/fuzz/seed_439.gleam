fn extends(constructor: Float, y: Float, value: Bool) -> Int {
100
}

pub fn main() {
  let rest = fn(v0) { "ab" }(0.25)
  let rest = {
    1.5
  } |> extends(0.1, {
    let rest = True
    rest
  })
  echo case 0 - rest, rest {
    3, 6 -> !{
      {
        let rest = rest
        True
      }
    }
    _, _ -> case 1 >= rest {
      a -> fn(v1, v2) { a }("abc", 2.0)
      _ -> True
    }
  }
  echo False
}
