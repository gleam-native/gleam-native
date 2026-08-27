pub const limit_value: String = "ab"
pub const golden_value: String = "b"
pub const pi_value: Bool = False

pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Cv4
}

pub type Promise {
  Cv5(value: String)
  Record(List(Int))
  Some(Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn extends(v6: String, v7: Float) -> Bool {
True
}

fn f1(item: Bool) -> Float {
{
    let y = case "constructor" != "b" {
      b -> "a" <> "b"
      _ -> "abc"
      False -> "b"
    }
    let item = "res" <> {
      "res" <> "a"
    }
    1.0
  }
}

fn constructor(v8: Float, v9: List(Int)) -> Bool {
{
    case <<"b":utf8, "constructor":utf8, 42:8>> {
      <<"ab":utf8, "abc":utf8, 3:1>> -> {
        let pair = v9
        let rest = v8
        "bc"
      }
      _ -> {
        let v8 = v8
        let this_ = v9
        "ab"
      }
    }
  } |> extends(100.0)
}

pub fn main() {
  let pi_value = {
    {
      let x = pi_value
      let rest = 4
      rest
    }
  } * 1
  echo golden_value
}
