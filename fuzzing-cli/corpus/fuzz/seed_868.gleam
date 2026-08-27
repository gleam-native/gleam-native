fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(acc: #(List(Int), Int)) -> List(Int) {
fn(v0, v1) { [10] }(42, 0.5)
}

fn delete(v2: Int, v3: List(Int), v4: Int) -> Float {
case [] {
    [3, 9, ..] as whole -> 0.0
    [x] -> {
      fn(v5) { 0.1 }(0)
    } *. {
      0.25
    }
    _ -> {
      {
        0.25
      } -. {
        0.5
      }
    } -. {
      1.5
    }
  }
}

fn f2(v6: String, constructor: String, class: Int) -> Bool {
True
}

pub fn main() {
  let self_ = 0.5
  let s = "ab"
  echo {
    case 0 + 10 {
      self_ -> 3 + self_
      9 -> 10 + 7
      v7 -> v7 * v7
    }
  } + {
    {
      let s = 1 >= 2
      0 * 0
    }
  }
  echo case <<"x":utf8>> {
    <<42:1>> -> case 0 {
      6 -> fn(v8, v9) { [5, 2] }(False, 1.0)
      v -> [0, 7]
      3 -> {
        let l = 1
        [2]
      }
    }
    <<5:16, constructor:big-unsigned-4>> as whole -> []
    _ -> case constructor(#([100, 42], 0)) {
      [self_, b, ..] if b <= 0 -> {
        let delete = [7, 1]
        let this_ = True
        [4, 3]
      }
      [] -> [0, 42]
      [_, 3, ..] -> {
        let self_ = self_
        []
      }
      _ -> fn(v10, v11) { [3] }(2, 100.0)
    }
  }
  echo case s == "res", s {
    item, _ -> case [4] {
      [_, ..rest] -> 3
      [] -> 2 + 4
      [6] -> fn(v12, v13) { 0 }(7, "res")
      _ -> 100 - 42
    }
    _, _ -> 10
  }
}
