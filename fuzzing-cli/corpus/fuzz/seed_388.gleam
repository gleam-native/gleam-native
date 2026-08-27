pub type Record {
  Cv0(value: String, inner: Int)
  Number
}

fn f0(l: Float) -> Float {
case {
      0.1
    } +. l, "b" <> "ab" {
    v1, "x" -> case <<"x":utf8>> {
      <<100:4>> -> fn(v2, v3) { v1 }(0, "ab")
      <<_:big-unsigned-16, value:big-unsigned-8>> as whole if value > 8 -> 10.0
      _ -> v1 -. {
        0.0
      }
    }
    _, "x" <> _ -> case 5 {
      item -> {
        0.25
      } -. l
      8 -> l
      6 | 5 -> l
    }
    _, v4 -> 3.14
  }
}

fn f1(constructor: Record) -> List(Int) {
[]
}

fn f2(v5: String) -> Bool {
False
}

pub fn main() {
  let acc = {
    {
      0.1
    } -. {
      100.0
    }
  } -. {
    0.1
  }
  echo []
  echo {
    fn(v6, v7) { v7 - 5 }("bc", 10)
  } - {
    {
      0 - 100
    } % 6
  }
  echo case "res" {
    "res" <> rest if rest == "" -> case fn(v8, v9) { Number }(42, 42) {
      v10 -> 0.5
      Cv0(_, item) -> acc /. {
        1.0
      }
    }
    "abc" <> rest -> case False {
      _ | False -> acc |> f0()
      a -> acc
    }
    _ -> f0(acc) |> f0()
  }
  echo 100
}
