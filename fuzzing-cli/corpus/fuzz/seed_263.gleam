pub const limit_value: Bool = True
pub const pi_value: Int = 3

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(List(Int), String), v0: #(Int, String), v1: Bool) -> Int {
case [], "x" {
    [_, 4, ..], "ab" <> rest if rest == "" && rest == "b" -> 7
    [6, a, ..], "constructor" <> rest -> case [] {
      [3] -> walk([], 100)
      [7] as whole -> 100
      [a, constructor, ..] as whole -> 10
      _ -> a - 5
    }
    v2, _ -> case 10.0, "ab" {
      rest, "a" -> 7 + 5
      1.5 as whole, v3 -> 1 % 6
      100.0 as whole, _ -> 42 - 100
      v4, _ -> {
        let self_ = False
        let y = 5
        y
      }
    }
  }
}

fn f1(v5: Int, pair: String) -> List(Int) {
[]
}

fn extends(delete: Bool, v6: Bool, value: Bool) -> Bool {
{
    {
      fn(v7) { 0.25 }(False)
    } *. {
      {
        0.5
      } -. {
        3.14
      }
    }
  } != {
    case 5, {
        let y = 1
        let default = 42
        ""
      } {
      9, "ab" -> 100.0
      v8, "bc" <> rest -> {
        let y = [2]
        3.14
      }
      _, v9 -> {
        3.14
      } +. {
        0.25
      }
    }
  }
}

pub fn main() {
  let limit_value = pi_value
  echo True
  echo extends(case fn(v10, v11) { 10 }("constructor", True), #(10, 2.0) {
    7, #(2, 3.14) as whole -> extends(True, False, False)
    v12, #(3, 0.1) if v12 == 0 && v12 > 6 -> True
    5, #(_, _) -> False
    _, _ -> extends(True, False, True)
  }, False, case {
      let new = True
      #("bc", "data")
    } {
    #(_, "abc") -> extends(True, True, True)
    #(_, "ab") | #("data", "bc" <> _) -> {
      let pi_value = [5, 1]
      let pi_value = []
      True
    }
    v13 -> False
  })
  echo False
  echo {
    10.0
  } /. {
    0.5
  }
}
