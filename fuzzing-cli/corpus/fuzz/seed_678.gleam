pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Int) -> String {
case walk([10], 5) {
    _ -> case [] {
      [9] -> "a" <> "b"
      [_] as whole -> "bc"
      v3 -> "ab" <> "abc"
    }
    9 -> "constructor"
  }
}

fn f1(x: Bool) -> Float {
1.5
}

pub fn main() {
  let constructor = [4]
  echo {
    case #(True, [3, 2]), 2 {
      #(True, [8]) as whole, _ -> constructor
      #(False, [_, ..rest]) as whole, _ -> {
        let rest = "data"
        let v = 2.0
        [4]
      }
      _, _ -> constructor
    }
  } |> walk(walk(constructor, 0))
  echo True
}
