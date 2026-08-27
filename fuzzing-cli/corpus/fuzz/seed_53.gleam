fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Float, class: Float, new: Bool) -> String {
{
    fn(v0, v1) { "" }(2.0, False)
  } <> {
    {
      let m = fn(v2, v3) { [5, 42] }("a", True)
      "constructor"
    }
  }
}

pub fn main() {
  echo True
  echo case 4 - 3 {
    inner -> fn(v4) { fn(v5, v6) { "a" }(True, 0.1) }(1.0)
    constructor -> case [0], constructor {
      [4, ..rest], v7 if v7 > 7 && v7 <= 6 -> f0(1.5, 0.5, True)
      [7, ..rest], 2 -> "ab"
      _, _ -> "abc" <> "constructor"
    }
    9 | 0 -> "" <> "abc"
  }
  echo {
    case {
        let l = "ab"
        l
      } {
      "ab" <> rest -> {
        let arguments = 0.1
        arguments
      }
      "b" -> {
        10.0
      } -. {
        0.0
      }
      inner | "abc" <> inner -> 0.1
    }
  } *. {
    {
      {
        0.25
      } +. {
        100.0
      }
    } -. {
      0.5
    }
  }
}
