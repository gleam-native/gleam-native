pub const pi_value: Int = 4

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Int)
}

pub type V3 {
  Cv4(value: Int, inner: Bool)
}

pub type V5 {
  Error
  Cv6
  Cv7(Int, Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v8: Bool, class: Bool, v9: String) -> Float {
case v9, {
      let v9 = v9
      3
    } {
    "constructor" as whole, 5 -> case Cv6 {
      Cv7(_, 3) | Cv6 -> {
        let s = 10.0
        let z = [42, 0]
        s
      }
      Cv7(inner, _) -> {
        10.0
      } -. {
        100.0
      }
      Cv6 | Error -> 0.25
      v10 -> {
        1.5
      } +. {
        10.0
      }
    }
    "abc", _ -> {
      0.5
    } *. {
      {
        100.0
      } +. {
        3.14
      }
    }
    "a", 2 -> 0.0
    _, _ -> 3.14
  }
}

fn f1(v11: Bool) -> Bool {
{
    fn(v12) { "abc" <> "res" }(3.14)
  } != "data"
}

pub fn main() {
  echo {
    fn(v13, v14) { v13 <> "x" }("abc", True)
  } <> {
    case 0 {
      _ -> "a"
      item -> "res" <> "abc"
      3 | 9 -> "abc"
    }
  }
  echo case fn(v15) { 100.0 }("data") {
    inner -> "a"
    inner -> {
      let self_ = f0(True, True, "")
      let pair = True |> f0(True |> f1(), "a")
      "x" <> "a"
    }
  }
}
