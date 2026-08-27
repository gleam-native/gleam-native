pub type Symbol {
  Cv0(value: String, inner: String)
  Cv1(value: String, inner: Bool)
  Cv2(value: Float, inner: Float)
}

pub type Record {
  Some
}

fn new(rest: Bool) -> List(Int) {
fn(v3) { [0] }(7)
}

fn f1(v4: Int, delete: Int, rest: Int) -> Float {
{
    case {
        let l = False
        7
      } {
      v5 -> 2.0
      v6 -> {
        3.14
      } *. {
        0.1
      }
      v7 -> 0.5
    }
  } -. {
    0.0
  }
}

pub fn main() {
  echo new({
    {
      let m = 42
      let y = True
      False
    }
  } && {
    {
      100.0
    } <=. {
      100.0
    }
  })
  echo {
    {
      fn(v8) { v8 }(10.0)
    } *. {
      {
        0.25
      } -. {
        0.5
      }
    }
  } -. {
    fn(v9) { {
      10.0
    } +. {
      3.14
    } }(True)
  }
  echo case Cv2(0.1, 0.1), 2 - 10 {
    Cv2(0.5, _), 5 -> case "a", "res" {
      "res", "res" <> _ -> {
        2.0
      } == {
        0.0
      }
      "constructor", self_ -> {
        1.5
      } == {
        3.14
      }
      _, _ -> "a" != ""
    }
    _, acc -> case 42 % 3 {
      2 -> True
      item -> "res" != "bc"
      item -> False
    }
    Cv0(_, "data"), _ -> fn(v10) { {
      let y = True
      let class = [3, 42]
      y
    } }("constructor")
  }
}
