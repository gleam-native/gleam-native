pub const limit_value: Int = 3

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Record(value: String)
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(z: String, constructor: Bool) -> Int {
{
    case {
        let length = z
        10
      } {
      a -> a
      7 | 2 -> 0
    }
  } + {
    case z <> "b" {
      "a" -> 100
      "res" <> rest -> 7
      "constructor" <> rest -> walk([2, 0], 7)
      _ -> {
        let default = 100
        default
      }
    }
  }
}

fn f1(item: Int, v4: V0) -> Float {
{
    case {
        10.0
      } -. {
        2.0
      } {
      inner -> inner
      1.5 as whole -> whole
      constructor -> 2.0
    }
  } /. {
    2.0
  }
}

fn f2(arguments: V0) -> List(Int) {
fn(v5) { case 42, 10 {
    v6, v7 -> {
      let arguments = v5
      let n = [5, 2]
      n
    }
    6, _ -> [7, 7]
    9 as whole, v8 -> []
  } }(3)
}

pub fn main() {
  let limit_value = case fn(v9, v10) { limit_value }(1.5, 1.0) {
    _ -> {
      let z = False
      ""
    }
    9 | 6 -> "b"
    4 | 5 -> "data" <> "ab"
  }
  let limit_value = fn(v11) { 10.0 }(False)
  echo f0("b", True) % 1
  echo "x"
}
