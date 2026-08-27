fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, s: Int, self_: List(Int)) -> Float {
case True, "constructor" {
    True, "ab" -> case fn(v0) { self_ }(100.0), walk(self_, 4) {
      [3, ..rest], self_ -> fn(v1) { 10.0 }("bc")
      [_], 5 -> 0.25
      _, _ -> 10.0
    }
    True, "abc" as whole -> {
      0.25
    } +. {
      {
        2.0
      } -. {
        0.25
      }
    }
    True, _ -> case "ab" {
      "bc" -> {
        0.1
      } *. {
        10.0
      }
      "abc" | "abc" <> _ -> fn(v2) { 0.5 }(True)
      _ -> {
        let self_ = self_
        3.14
      }
    }
    _, _ -> case {
        let y = [3, 0]
        let m = "data"
        y
      } {
      [] -> 10.0
      [_] -> 0.0
      [] -> fn(v3) { 0.1 }(4)
      _ -> {
        0.1
      } +. {
        1.0
      }
    }
  }
}

fn f1(v4: Bool, v5: Int) -> List(Int) {
[7]
}

fn f2(v6: Int, new: Float) -> List(Int) {
case walk([100, 10], 1), 4 {
    5, _ -> True |> f1(2)
    _, 6 -> f1({
      let v6 = True
      let s = []
      True
    }, v6 - 7)
    4, 1 -> f1(True, v6 + v6)
    _, v7 -> [100]
  }
}

pub fn main() {
  echo True |> f1(fn(v8) { 42 }(True))
  echo {
    "bc" <> "b"
  } <> "res"
  echo {
    let item = case "x", 3 + 1 {
      "data", 3 -> "res" == ""
      "res" <> rest, 9 -> True
      _, _ -> False || True
    }
    case "b", [100, 7] |> walk(5) {
      "abc", _ -> True |> f0(10 - 4, [])
      "x" <> rest, constructor -> 3.14
      _, v9 -> 0.1
    }
  }
}
