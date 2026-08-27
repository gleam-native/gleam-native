fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(y: Bool) -> List(Int) {
[7]
}

fn f1(pair: Int, class: Int, v0: #(Bool, Float)) -> List(Int) {
[42, 3]
}

fn default(acc: Int) -> Int {
case "data" <> "res", fn(v1) { False }(1.0) {
    _, _ -> acc
    "b", acc -> case {
        let new = acc
        let acc = 100.0
        new
      } {
      v2 -> 7 * 42
      v3 -> 4
    }
  }
}

pub fn main() {
  echo 1.0
  echo 0
  echo {
    {
      {
        3.14
      } +. {
        0.25
      }
    } *. {
      {
        0.1
      } *. {
        2.0
      }
    }
  } == {
    10.0
  }
}
