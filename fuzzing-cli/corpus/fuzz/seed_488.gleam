pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type Object {
  Cv2(value: List(Int))
  Cv3(Float)
  Cv4
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v5: V0) -> Int {
[] |> walk(5)
}

fn static(v: List(Int), s: Float, new: Float) -> Float {
case {
      let value = 0.1
      "data"
    }, #(2, True) {
    "abc" <> _, #(_, False) -> {
      fn(v6) { new }(10)
    } -. {
      new +. {
        0.25
      }
    }
    "bc", #(_, _) -> case [] {
      [] -> {
        0.1
      } *. {
        2.0
      }
      [7, 8, ..] -> s
      [1, b, ..] -> fn(v7) { new }(0.0)
      _ -> {
        let v = False
        let length = s
        length
      }
    }
    v8, v9 -> {
      0.25
    } *. s
  }
}

pub fn main() {
  echo []
  echo "constructor"
  echo {
    [1, 7] |> walk({
      let value = 4
      let m = False
      value
    })
  } + {
    case 0.1 {
      _ -> 4
      x -> 0
      a -> 7
    }
  }
  echo {
    {
      {
        0.1
      } -. {
        0.1
      }
    } -. {
      1.5
    }
  } == {
    case 5 {
      item -> {
        0.1
      } *. {
        0.5
      }
      _ | 2 -> static([], 1.0, 2.0)
      v10 -> static([7, 0], 0.1, 0.1)
    }
  }
}
