pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(m: Bool) -> Float {
2.0
}

fn f1(v: Bool, v2: Bool) -> Bool {
case 3 {
    9 -> case "" <> "constructor" {
      "constructor" <> item | "constructor" <> item -> v2 || False
      "bc" -> {
        let x = 10
        let v = []
        False
      }
      constructor -> fn(v3) { v }("res")
    }
    3 -> case fn(v4) { "a" }(1) {
      "x" -> {
        0.1
      } != {
        100.0
      }
      "res" -> v
      constructor | "abc" <> constructor -> True
    }
    v5 -> v
  }
}

pub fn main() {
  let new = 7
  echo {
    "b" != "a"
  } |> f1({
    let delete = "bc"
    False
  })
  echo f1(True |> f1(True), fn(v6, v7) { False }("", 10))
  echo {
    1.5
  } <=. {
    {
      1.0
    } +. {
      {
        1.5
      } +. {
        1.0
      }
    }
  }
  echo {
    let new = [5]
    case True && False {
      True -> new
      _ -> new
    }
  }
}
