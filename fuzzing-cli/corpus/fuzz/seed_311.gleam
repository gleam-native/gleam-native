pub const seed_value: Int = 0

pub type V0 {
  Record(value: String, inner: List(Int))
}

pub type V1 {
  Cv2(List(Int), value: Int)
  Number
}

pub type Promise {
  Cv3
}

fn f0(m: Int, n: Int, v4: Int) -> String {
"b" <> {
    "data" <> {
      {
        let n = v4
        let y = "constructor"
        "b"
      }
    }
  }
}

fn f1(z: #(Int, Float), class: Int, value: #(Int, String)) -> Float {
{
    case {
        let value = class
        let pair = "constructor"
        False
      } {
      v5 -> 0.1
      True -> fn(v6, v7) { 0.1 }(2.0, "constructor")
    }
  } -. {
    case 10 {
      _ -> 1.5
      a -> {
        0.25
      } -. {
        0.5
      }
      0 | 4 -> {
        let length = 2.0
        length
      }
    }
  }
}

fn f2(v8: Int, v9: Promise, delete: Int) -> Float {
3.14
}

pub fn main() {
  let seed_value = []
  let seed_value = case {
      let seed_value = 2
      let seed_value = [100]
      42
    } {
    _ -> fn(v10, v11) { False }(1, 5)
    4 | 1 -> {
      let l = 5
      True
    }
  }
  echo f2(case #([], "x"), 100 * 10 {
    #([], v12), 0 if v12 == "bc" -> 7 % 3
    #([], "b"), y -> 2 + y
    v13, _ -> 0
  }, Cv3, {
    0 - 5
  } + {
    {
      let class = "ab"
      42
    }
  })
  echo {
    fn(v14, v15) { 5 % 6 }(2, 7)
  } |> f0(100, 7)
}
