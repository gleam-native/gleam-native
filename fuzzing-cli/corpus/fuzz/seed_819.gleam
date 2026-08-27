pub const tag_value: String = "abc"
pub const pi_value: Float = 1.0

pub type V0 {
  None(value: String, inner: Float)
  Cv1(value: String, inner: String)
}

pub type Number {
  Record(Float)
  Cv2(List(Int))
}

pub type V3 {
  Cv4(Bool)
  Cv5(Float)
  Cv6(value: List(Int))
}

fn f0(new: Number, m: Int, z: Int) -> List(Int) {
{
    let value = [2]
    [2, 3]
  }
}

fn f1(prototype: #(Float, Int)) -> Float {
case "bc" {
    "constructor" -> case "a" {
      constructor -> 0.25
      _ -> {
        let prototype = 1
        3.14
      }
    }
    "a" | "bc" <> _ -> 3.14
    v7 -> case 100 * 3 {
      _ -> {
        0.0
      } -. {
        3.14
      }
      0 as whole if whole == 4 -> {
        1.0
      } /. {
        2.0
      }
      _ -> 100.0
    }
  }
}

fn extends(n: #(List(Int), Bool), v: List(Int)) -> String {
case "b" <> "" {
    v -> v
    "abc" -> case {
        let n = True
        []
      }, Cv2([]) |> f0(fn(v8) { 10 }(False), 0) {
      [5, v, ..], [3] as whole -> ""
      [_, ..rest] as whole, [4, ..tail] as it -> "a"
      v9, _ -> "x"
    }
  }
}

pub fn main() {
  let acc = "ab"
  echo {
    let v = {
      42 - 4
    } % 3
    let self_ = case v > v, Record(10.0) {
      True as whole, Cv2([4]) -> [1]
      _, Record(0.1) -> []
      _, Cv2([9, ..rest] as whole) -> rest
      v10, _ -> []
    }
    extends(#([0], True), self_)
  }
  echo True
  echo {
    #(3.14, 42) |> f1()
  } >=. {
    0.0
  }
  echo {
    pi_value -. {
      {
        let length = []
        let length = "a"
        pi_value
      }
    }
  } +. {
    case [] {
      [2, 2, ..] -> {
        1.0
      } /. {
        3.14
      }
      [4] -> {
        100.0
      } *. {
        1.0
      }
      [] -> #(0.5, 1) |> f1()
      v11 -> pi_value -. {
        0.5
      }
    }
  }
}
