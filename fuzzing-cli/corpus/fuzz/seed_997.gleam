pub const limit_value: String = ""
pub const golden_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: List(Int), s: Int) -> List(Int) {
case fn(v1) { v0 }(3), True {
    [3, 7, ..], False -> case True {
      _ -> v0
      False | False -> {
        let pair = s
        let delete = False
        v0
      }
    }
    [b, 9, ..] as whole, True -> case 0.25 {
      a -> [5]
      0.25 -> v0
    }
    [8, 8, ..], _ -> case {
        let s = s
        constructor
      }, [100, 5] |> walk(walk(v0, 1)) {
      _, 2 as whole if whole <= 6 -> []
      "ab", 6 -> [3]
      _, v2 -> [0]
    }
    _, _ -> [7]
  }
}

fn f1(v3: Bool) -> Bool {
case f0("res", [], 4) {
    [] -> v3
    [_, ..rest] -> 7 > 100
    v4 -> case <<4:4>> {
      <<"b":utf8>> as whole -> True || v3
      <<_:utf8>> -> {
        0.5
      } == {
        1.5
      }
      <<"":utf8>> -> {
        3.14
      } >=. {
        3.14
      }
      _ -> fn(v5, v6) { v6 }(True, False)
    }
  }
}

pub fn main() {
  let class = "ab"
  echo case "bc", fn(v7, v8) { 3.14 }(7, 5) {
    "data" <> _, _ -> {
      {
        0.0
      } *. {
        2.0
      }
    } -. {
      {
        2.0
      } +. {
        0.5
      }
    }
    _, _ -> 2.0
  }
  echo False
  echo {
    fn(v9, v10) { 100.0 }(0.25, 10)
  } >=. {
    case {
        let constructor = 100
        class
      } {
      "abc" as whole -> 0.5
      v11 | "x" <> v11 -> {
        100.0
      } +. {
        1.0
      }
    }
  }
}
