pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: Int, this_: Int, v2: Float) -> Bool {
case {
      0.5
    } +. v2 {
    b -> True
    _ -> case 0 - this_, fn(v3) { "constructor" }(True) {
      s, "constructor" <> rest -> {
        let acc = 0.1
        let self_ = []
        False
      }
      3 as whole, "data" <> rest if whole > 9 -> True
      _, "res" -> {
        10.0
      } <=. {
        1.5
      }
      v4, _ -> False
    }
  }
}

pub fn main() {
  let pair = 42
  echo {
    {
      0.0
    } /. {
      1.0
    }
  } -. {
    case #(10, 42) {
      #(pair, _) if pair <= 6 -> 3.14
      b -> 0.25
    }
  }
  echo {
    let default = [7, 10]
    walk(default, pair) + pair
  }
}
