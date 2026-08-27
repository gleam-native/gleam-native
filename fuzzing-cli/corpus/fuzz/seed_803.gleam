pub type V0 {
  Error(value: String, inner: String)
}

pub type V1 {
  Ok(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: Bool, value: V1) -> Float {
case [100] |> walk(1 - 3), 42 {
    3, _ -> case fn(v2, v3) { [2, 1] }(True, True) {
      [5, constructor, ..] -> 0.1
      [7, ..rest] -> {
        0.1
      } +. {
        3.14
      }
      [] -> {
        0.0
      } /. {
        0.5
      }
      _ -> {
        3.14
      } -. {
        1.0
      }
    }
    _, v4 -> case {
        let arguments = v4
        let acc = 100
        100
      } {
      _ -> 0.5
      v5 -> fn(v6) { 0.5 }(1.5)
      b -> 0.0
    }
    8 as whole, 0 -> case 5 == 42 {
      True -> fn(v7, v8) { v7 }(0.0, True)
      a -> 0.25
    }
  }
}

fn class(delete: #(List(Int), List(Int)), v9: Int, v10: String) -> Bool {
{
    v9 % 4
  } >= v9
}

fn new(self_: Float) -> Float {
3.14
}

pub fn main() {
  let item = 7
  echo {
    let v = case fn(v11) { 1.5 }(10) {
      3.14 -> ""
      rest -> "data"
    }
    {
      item - 42
    } - item
  }
}
