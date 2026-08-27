pub const tag_value: Int = 1

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v4: V2, v5: Int) -> Int {
{
    let arguments = 1.0
    let m = case Cv1, [100] {
      _, [5, a, ..] -> {
        let x = 0.5
        True
      }
      _, [4, 3, ..] -> False
      _, _ -> "constructor" == "x"
    }
    {
      {
        let this_ = []
        4
      }
    } - {
      v5 - v5
    }
  }
}

fn f1(arguments: Float, y: Bool, rest: V0) -> Bool {
{
    case 3 == 7 {
      constructor -> 0.25
      _ -> arguments +. arguments
      constructor -> {
        10.0
      } -. arguments
    }
  } >=. {
    {
      let pair = fn(v6, v7) { [42, 100] }(4, 100)
      let y = arguments
      arguments +. {
        0.1
      }
    }
  }
}

fn f2(v8: Int) -> Int {
v8
}

pub fn main() {
  echo case #(2.0, [4]) {
    constructor -> case [1] {
      [] -> tag_value == 4
      [b] if b <= 5 -> fn(v9) { True }(False)
      [b, 3, ..] -> True
      _ -> {
        1.5
      } <. {
        0.0
      }
    }
    #(1.0, [4, constructor, ..]) as whole -> True
    #(3.14, [9, ..rest]) -> case {
        let z = 0.5
        let delete = rest
        True
      } {
      item -> item
      item -> item
    }
  }
}
