pub const tag_value: Int = 5
pub const limit_value: Int = 10

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: Int) -> List(Int) {
case "ab" {
    "a" <> _ as whole if whole != "ab" || whole != "data" -> {
      let class = 0.25
      let l = [42, 100]
      l
    }
    "data" <> _ -> [4]
    "ab" <> b | "data" <> b -> {
      let class = 100
      fn(v2) { [] }(True)
    }
    _ -> []
  }
}

fn export(v3: Float, v: Int, value: V0) -> Float {
{
    0.25
  } +. {
    case value {
      Cv1([]) | Cv1(_) -> v3
      _ -> v3 +. {
        1.0
      }
    }
  }
}

pub fn main() {
  echo {
    1.5
  } <=. export({
    0.0
  } |> export(5, Cv1([])), 10, Cv1([0, 42]))
  echo limit_value - {
    walk([0], tag_value) - {
      {
        let limit_value = 0.5
        tag_value
      }
    }
  }
  echo case Cv1([3]), {
      let default = tag_value
      let class = []
      True
    } {
    _, True -> True
    _, _ -> {
      {
        0.0
      } != {
        1.0
      }
    } || {
      {
        let limit_value = [7, 42]
        False
      }
    }
  }
  echo {
    case "a", 100.0 {
      "" <> _, 100.0 -> "abc" <> "data"
      "x" <> _, 10.0 -> fn(v4) { "a" }(False)
      "bc", 0.5 -> "b" <> "res"
      v5, _ -> "a"
    }
  } <> {
    "abc" <> {
      fn(v6, v7) { "constructor" }(True, 7)
    }
  }
}
