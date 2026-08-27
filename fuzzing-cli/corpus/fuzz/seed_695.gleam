pub const tag_value: Bool = True
pub const euler_value: Bool = True
pub const limit_value: Float = 0.25

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(n: Float) -> Bool {
{
    let n = [2, 7]
    case 2 == 2, True {
      v0, False -> v0
      _, True -> {
        let n = False
        True
      }
      _, False -> {
        let default = [0, 3]
        False
      }
      v1, _ -> {
        0.5
      } <=. {
        2.0
      }
    }
  }
}

fn static(class: List(Int)) -> Float {
{
    {
      fn(v2) { 0.1 }("constructor")
    } *. {
      {
        1.0
      } /. {
        3.14
      }
    }
  } +. {
    {
      2.0
    } -. {
      {
        100.0
      } /. {
        0.5
      }
    }
  }
}

fn f2(arguments: #(String, List(Int)), v3: Float, m: #(String, Bool)) -> String {
{
    {
      let m = 1.5
      let arguments = True
      "a" <> "ab"
    }
  } <> {
    {
      let m = 7 + 100
      "abc" <> "ab"
    }
  }
}

pub fn main() {
  let limit_value = "data"
  echo {
    100.0
  } -. {
    0.25
  }
  echo [7]
}
