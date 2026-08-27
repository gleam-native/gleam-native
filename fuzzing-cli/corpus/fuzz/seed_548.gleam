pub type Record {
  Cv0(value: String, inner: List(Int))
  Number(List(Int), value: Float)
  Cv1(value: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(v2: Int, v3: Float, v4: Bool) -> List(Int) {
fn(v5) { [4] }(0)
}

fn f1(v6: List(Int)) -> Bool {
case {
      let v6 = v6
      let v6 = True
      Cv0("res", [])
    } {
    v7 -> case "constructor" {
      item | "" <> item -> fn(v8, v9) { v9 }(3, False)
      _ -> {
        let z = v6
        let v7 = [5, 100]
        True
      }
      "constructor" -> False
    }
    new -> case 4, False && False {
      v6, True -> 5 != v6
      6, _ -> False || True
      _, _ -> False
    }
  }
}

fn f2(v: Bool, m: Int) -> String {
{
    case {
        let m = v
        42
      } {
      1 -> "res" <> "a"
      _ -> "b" <> "ab"
    }
  } <> {
    case Cv1(100.0), 1.0 {
      Cv1(0.25 as whole) as it, _ -> "abc"
      Cv0(_, []), _ -> fn(v10) { "b" }("a")
      Number([7] as whole, 3.14), 2.0 -> "res"
      _, _ -> fn(v11, v12) { "res" }(False, True)
    }
  }
}

pub fn main() {
  echo 5
  echo walk([10, 10], 100)
  echo {
    case [3] {
      [5] -> {
        let constructor = 0.25
        let acc = 1
        3.14
      }
      [6, ..rest] -> 0.0
      v13 -> {
        100.0
      } /. {
        0.5
      }
    }
  } +. {
    0.1
  }
  echo case #("bc", 3), class(1, 0.0, True) {
    #(_, _), [a, 1, ..] if a > 9 -> 100.0
    #("x", 4) as whole, [] as it -> 1.5
    _, v14 -> 10.0
  }
}
