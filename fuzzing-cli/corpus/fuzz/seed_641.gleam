pub const limit_value: Bool = True
pub const tag_value: String = "bc"
pub const golden_value: Float = 1.0

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(this_: Int) -> Float {
{
    {
      let this_ = {
        3.14
      } *. {
        1.5
      }
      {
        0.5
      } +. {
        1.5
      }
    }
  } *. {
    {
      {
        0.5
      } /. {
        1.0
      }
    } +. {
      2.0
    }
  }
}

fn f1(v0: Bool, v1: String) -> Int {
{
    {
      1 - 100
    } + {
      {
        let acc = 0
        let value = "x"
        acc
      }
    }
  } - {
    case fn(v2, v3) { v0 }(2, True) {
      b -> [] |> walk(3)
      True | True -> 1
      _ -> 42
    }
  }
}

fn f2(v4: #(Int, Bool), prototype: #(String, Bool)) -> Float {
{
    {
      {
        1.5
      } *. {
        100.0
      }
    } -. constructor(2)
  } +. {
    fn(v5) { {
      let length = 0
      let y = 3
      v5
    } }(0.0)
  }
}

pub fn main() {
  echo fn(v6) { 1.0 }(3.14)
  echo {
    let n = 42 - {
      42 + 4
    }
    True
  }
  echo fn(v7) { {
    let s = tag_value <> ""
    let y = 1.5
    [100]
  } }(100)
}
