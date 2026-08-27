pub const limit_value: Int = 3
pub const euler_value: Int = 42
pub const tag_value: Float = 0.0

pub type V0 {
  Number(value: String, inner: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: String) -> Int {
7
}

fn constructor(acc: V0) -> Float {
{
    {
      {
        1.0
      } +. {
        100.0
      }
    } /. {
      2.0
    }
  } +. {
    3.14
  }
}

pub fn main() {
  let value = case "res" {
    _ -> 100.0
    "data" <> a -> {
      100.0
    } *. tag_value
  }
  echo constructor(Number("abc", False)) +. {
    case "bc" <> "data" {
      "bc" <> _ -> {
        0.5
      } +. value
      "data" <> _ -> 10.0
      "a" <> _ -> {
        10.0
      } *. {
        3.14
      }
      _ -> value
    }
  }
  echo {
    case euler_value {
      _ -> "" <> "a"
      constructor -> "" <> "constructor"
    }
  } <> {
    {
      let delete = 3 - limit_value
      let acc = "res" <> "abc"
      acc
    }
  }
}
