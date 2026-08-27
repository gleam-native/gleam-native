pub type V0 {
  Cv1
}

pub type V2 {
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn export(v4: Int) -> Float {
case Cv1 {
    Cv1 -> 0.25
    b -> {
      {
        let self_ = True
        let self_ = v4
        1.0
      }
    } *. {
      {
        0.0
      } *. {
        0.1
      }
    }
  }
}

pub fn main() {
  echo {
    let y = 5
    7
  }
  echo case "b", spin(1, 2) {
    "x", _ -> !{
      fn(v5) { True }("bc")
    }
    _, 9 as whole if whole > 7 && whole > 5 -> {
      0.25
    } == {
      1.5
    }
    _, pair -> True
  }
}
