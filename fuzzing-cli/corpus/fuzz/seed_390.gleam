pub type V0 {
  None(value: String, inner: Int)
}

pub type V1 {
  Cv2
  Cv3(List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v4: String) -> String {
case v4 {
    _ -> {
      "bc" <> v4
    } <> {
      v4 <> ""
    }
    "abc" <> constructor | "" <> constructor -> {
      let constructor = fn(v5) { v5 }(10)
      v4
    }
    "data" <> _ -> v4
  }
}

fn extends(m: Bool) -> Bool {
m
}

fn class(v6: V0, prototype: List(Int), item: Bool) -> Bool {
case prototype {
    [_] -> item
    [b, 5, ..] -> {
      {
        let rest = []
        let x = False
        b
      }
    } == walk(prototype, 1)
    v7 -> False
  }
}

pub fn main() {
  let v = 1.5
  let item = False
  echo case fn(v8, v9) { 1.5 }(True, 1) {
    1.0 | 0.1 -> case {
        let y = [7]
        "x"
      } {
      inner -> {
        let n = inner
        [42, 0]
      }
      _ -> [5, 2]
    }
    1.5 -> fn(v10, v11) { [100] }(1.5, "x")
    _ -> {
      let item = v -. {
        100.0
      }
      [7]
    }
  }
  echo {
    let new = {
      "ab" |> f0()
    } <> {
      "b" <> "x"
    }
    let arguments = new
    {
      let new = 5
      let new = new + 10
      {
        let this_ = 0.1
        let constructor = this_
        []
      }
    }
  }
}
