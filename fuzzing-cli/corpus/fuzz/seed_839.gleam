pub const pi_value: Float = 2.0

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(constructor: V0) -> Bool {
False
}

fn f1(s: #(Int, Bool)) -> List(Int) {
[0]
}

pub fn main() {
  let rest = 3
  echo case "b" {
    "constructor" -> rest
    "ab" | "bc" -> case 7 {
      2 | 0 -> rest
      6 | 3 -> 0
      _ -> rest + rest
    }
    b -> case rest {
      3 -> 42
      8 -> 0
      _ -> 10
    }
  }
  echo case !False {
    v2 -> {
      pi_value +. pi_value
    } != {
      pi_value -. {
        1.5
      }
    }
    item -> {
      fn(v3) { Cv1 }("a")
    } |> static()
    True -> True
  }
  echo {
    case Cv1 {
      delete -> 0 * rest
      item -> rest
    }
  } * rest
  echo case Cv1, Cv1 {
    Cv1, rest -> "constructor"
    Cv1, Cv1 -> case #([], 2.0) {
      constructor -> "abc" <> ""
      b -> {
        let z = "res"
        ""
      }
      constructor -> "data"
    }
    v4, _ -> {
      "x" <> "b"
    } <> "b"
  }
}
