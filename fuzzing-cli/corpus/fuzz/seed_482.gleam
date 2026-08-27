pub const seed_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(arguments: String) -> Bool {
{
    !True
  } || True
}

fn f1(m: #(Float, Int)) -> Int {
case {
      let pair = [4]
      let value = 0
      pair
    } {
    [constructor] -> 2
    [] -> walk([10], [] |> walk(3))
    _ -> 5 + {
      {
        let length = 3
        100
      }
    }
  }
}

pub fn main() {
  let seed_value = [7, 3]
  let self_ = {
    {
      2.0
    } /. {
      1.0
    }
  } /. {
    2.0
  }
  echo seed_value
  echo case 0.1, fn(v0) { #(0.1, "data") }(2.0) {
    10.0, #(0.0, "ab") -> {
      let acc = constructor("constructor")
      []
    }
    _, #(1.0, _) -> seed_value
    v1, v2 -> fn(v3, v4) { fn(v5) { seed_value }(1.0) }(True, False)
  }
  echo {
    {
      2 - 0
    } % 2
  } % 3
}
