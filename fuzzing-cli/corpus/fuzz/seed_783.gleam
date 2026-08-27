fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(l: #(Int, Bool)) -> Float {
{
    case False || True {
      inner -> 3.14
      False | True -> {
        0.0
      } +. {
        10.0
      }
      _ -> {
        0.1
      } +. {
        3.14
      }
    }
  } -. {
    fn(v0) { {
      0.1
    } -. {
      0.25
    } }("ab")
  }
}

fn f1(m: Float, arguments: Int, self_: String) -> Float {
#(3, False) |> constructor()
}

fn f2(prototype: Int) -> String {
case {
      let constructor = 0.1
      let l = 0
      "constructor"
    } {
    b -> case spin(4, prototype), 1 |> spin({
        let b = 1
        b
      }) {
      1, 9 -> "constructor"
      0, 5 -> "b" <> "b"
      _, _ -> "constructor" <> "res"
    }
    "bc" <> rest | "constructor" <> rest -> rest
  }
}

pub fn main() {
  let x = {
    let v = []
    let v = {
      2.0
    } >=. {
      0.5
    }
    {
      let rest = v
      let rest = [42]
      "abc"
    }
  }
  let x = x
  echo case fn(v1) { x }(42) {
    _ | "x" <> _ -> case 4 >= 4 {
      default -> [2, 0]
      n -> [5]
    }
    "constructor" <> rest -> [4]
  }
}
