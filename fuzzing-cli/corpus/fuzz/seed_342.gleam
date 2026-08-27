fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: Float, v1: List(Int)) -> Float {
{
    let n = case walk(v1, 1), v0 +. {
        3.14
      } {
      _, _ -> []
      0, delete -> [4, 100]
      4, 10.0 -> [2]
    }
    v0
  }
}

fn f1(v2: Int, acc: #(Bool, List(Int))) -> String {
{
    let n = 4 % 1
    "a"
  }
}

pub fn main() {
  echo fn(v3) { {
    let v3 = v3
    "data" |> f0({
      0.1
    } *. {
      0.25
    }, [7, 5])
  } }(True)
  echo case {
      let l = "abc"
      [2, 4]
    } {
    [1] -> case "abc" {
      _ | "x" <> _ -> "data"
      prototype -> prototype
      inner -> "x" <> inner
    }
    [] -> "ab" <> {
      100 |> f1(#(True, [5, 42]))
    }
    v4 -> "ab"
  }
  echo {
    let new = True
    let rest = case {
        100.0
      } != {
        1.5
      } {
      constructor -> {
        let new = 4
        "constructor"
      }
      item -> f1(1, #(True, []))
      y -> 7 |> f1({
        let acc = True
        let new = []
        #(True, [])
      })
    }
    case {
        let self_ = []
        let l = 4
        l
      } {
      _ | 2 -> 1
      v5 -> 3 % 4
    }
  }
}
