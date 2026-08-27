pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Cv3(Int)
}

pub type Promise {
  Cv4
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v5: Int) -> Bool {
case {
      let new = "constructor"
      new
    }, {
      let default = True
      "x"
    } {
    "a", "bc" -> True
    "res", _ -> case #([100, 0], "b"), "x" {
      #([3] as whole, "data"), "x" <> _ as it -> True
      #([_, ..rest], "a"), "bc" <> tail as whole -> True
      _, v6 -> {
        let pair = v5
        let delete = []
        True
      }
    }
    _, v7 -> {
      2.0
    } == {
      {
        let rest = [1, 42]
        let v5 = v5
        1.0
      }
    }
  }
}

fn f1(class: Int, m: Promise, self_: List(Int)) -> Float {
{
    case #([], [4, 7]) {
      #([5, ..rest] as whole, [9]) -> {
        let prototype = 0.5
        1.5
      }
      #([_], [9, ..rest]) -> 1.5
      _ -> fn(v8) { 3.14 }(True)
    }
  } +. {
    100.0
  }
}

pub fn main() {
  let pair = case <<1:16, "":utf8>> {
    <<_:utf8, _:utf8>> -> !False
    <<"a":utf8>> -> {
      let arguments = [1, 1]
      let length = 100
      True
    }
    _ -> False
  }
  let v = case pair {
    False -> "data"
    _ | False -> ""
    True | True -> "a"
  }
  echo case fn(v9) { v }("bc") {
    "a" <> rest -> []
    "abc" | "constructor" -> {
      let n = pair
      let v = {
        1.0
      } +. {
        100.0
      }
      [2]
    }
    _ -> {
      let pair = f1(4, Cv4, [4, 100])
      let m = {
        let self_ = "abc"
        let arguments = pair
        v
      }
      []
    }
  }
  echo [2]
  echo {
    0.0
  } >=. {
    case fn(v10, v11) { v }(4, 0.1) {
      "abc" -> {
        0.25
      } -. {
        100.0
      }
      _ -> 2.0
      "a" -> {
        1.0
      } /. {
        1.0
      }
    }
  }
  echo {
    fn(v12) { [2] }(1.0)
  } |> walk(1)
}
