pub const tag_value: Float = 0.25

pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Cv4(value: String, inner: List(Int))
  Cv5(Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(arguments: Bool, prototype: Int, v6: Bool) -> String {
case {
      0.1
    } *. {
      10.0
    } {
    inner -> case <<"x":utf8>> {
      <<this_:8, 5:1>> if this_ > 6 -> "bc" <> "data"
      _ -> "x"
    }
    3.14 -> case #("ab", 0.25) {
      #("res", 10.0) | #("constructor", 1.0) -> {
        let new = "abc"
        new
      }
      #(_, _) -> "data" <> "bc"
      a -> "b"
    }
    _ -> "x"
  }
}

pub fn main() {
  let tag_value = fn(v7, v8) { True }("", False)
  echo {
    case <<"constructor":utf8, "res":utf8>> {
      <<3:8>> -> {
        3.14
      } *. {
        0.1
      }
      _ -> fn(v9, v10) { 0.0 }(0.1, True)
    }
  } +. {
    case {
        let value = "data"
        let value = value
        Cv1
      }, 42 % 7 {
      Cv2, 9 -> {
        0.0
      } +. {
        2.0
      }
      Cv1, 7 -> {
        3.14
      } +. {
        1.0
      }
      _, _ -> {
        3.14
      } +. {
        0.1
      }
    }
  }
  echo case {
      let tag_value = 4
      let tag_value = "res"
      Cv5(False)
    } {
    Cv5(_) | Cv5(_) -> case Cv1 {
      item -> {
        3.14
      } -. {
        0.25
      }
      Cv1 | Cv2 -> 0.1
      constructor -> 2.0
    }
    a -> {
      {
        3.14
      } *. {
        0.0
      }
    } /. {
      3.14
    }
  }
  echo 0
}
