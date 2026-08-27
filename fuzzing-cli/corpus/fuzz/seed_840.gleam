pub const limit_value: String = "constructor"
pub const tag_value: Float = 0.1
pub const pi_value: String = "x"

pub type V0 {
  Record(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(value: Int, v1: Bool) -> Float {
{
    {
      1.0
    } -. {
      {
        0.0
      } +. {
        2.0
      }
    }
  } *. {
    case "constructor" <> "constructor" {
      "a" <> rest | "data" <> rest -> {
        100.0
      } -. {
        0.25
      }
      b -> {
        2.0
      } -. {
        0.0
      }
      "abc" as whole -> {
        0.5
      } *. {
        0.25
      }
    }
  }
}

fn f1(v2: Bool, acc: Float, delete: Bool) -> String {
"abc"
}

pub fn main() {
  let acc = case [3] {
    [pi_value, 5, ..] if pi_value <= 7 -> True
    [] -> limit_value == pi_value
    [b, ..rest] -> tag_value <=. {
      10.0
    }
    _ -> True
  }
  echo acc
  echo case fn(v3) { acc }(True), tag_value <. tag_value {
    True, _ -> case "x" <> "a" {
      item | "res" <> item -> [3, 100]
      "abc" -> fn(v4) { [] }(0.1)
    }
    z, True as whole -> []
    _, v5 -> {
      let n = {
        1.5
      } +. tag_value
      [7]
    }
  }
  echo [3]
}
