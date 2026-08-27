pub const golden_value: Float = 1.0
pub const seed_value: Int = 10
pub const pi_value: Bool = False

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(List(Int))
  Record(value: String)
}

pub type V3 {
  Number(value: Int)
  Cv4(Int, value: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v5: #(Bool, Bool)) -> Float {
case {
      let v5 = True
      "ab"
    } {
    "x" -> {
      100.0
    } -. {
      10.0
    }
    "a" -> {
      3.14
    } /. {
      1.0
    }
    "constructor" -> case [2, 1] {
      [_, ..rest] as whole -> 0.1
      [_] -> 3.14
      _ -> {
        0.1
      } *. {
        1.5
      }
    }
    v6 -> {
      1.5
    } -. {
      1.0
    }
  }
}

fn f1(v7: V3, v8: Float) -> Float {
#(True, True) |> arguments()
}

fn f2(s: List(Int), self_: String, v9: Float) -> String {
case <<"a":utf8, 100:1, "bc":utf8>> {
    <<_:utf8>> -> {
      let v9 = 4 - 0
      {
        let item = "constructor"
        let v = 10
        "abc"
      }
    }
    _ -> {
      let v9 = {
        let delete = v9
        2
      }
      let s = 100
      {
        let v9 = self_
        v9
      }
    }
  }
}

pub fn main() {
  let new = 7
  let s = [2]
  echo {
    golden_value +. golden_value
  } +. {
    {
      let golden_value = s
      {
        10.0
      } +. {
        0.25
      }
    }
  }
  echo case s, walk([5], seed_value) {
    [h, ..rest] as whole, 6 if h <= 0 && h <= 8 -> {
      let m = {
        0.1
      } +. {
        100.0
      }
      s
    }
    [_] as whole, pi_value -> [7]
    _, v10 -> [5]
  }
  echo {
    {
      100.0
    } -. golden_value
  } -. arguments(#(True, False))
}
