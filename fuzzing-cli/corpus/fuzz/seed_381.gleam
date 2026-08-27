pub const tag_value: Bool = False
pub const euler_value: Bool = False

pub type Number {
  Cv0(value: String, inner: String)
  Error(value: String, inner: Int)
}

fn f0(default: List(Int)) -> Float {
case Error("res", 7) {
    v1 -> {
      2.0
    } -. {
      {
        let m = 10
        1.5
      }
    }
    Error("constructor" <> _, 5) -> case 1.5 {
      a -> 0.0
      _ -> {
        let length = "res"
        let length = length
        0.25
      }
    }
    Error(_, constructor) -> {
      {
        1.5
      } -. {
        0.1
      }
    } +. {
      {
        0.0
      } *. {
        0.0
      }
    }
  }
}

fn f1(class: List(Int)) -> Float {
10.0
}

pub fn main() {
  let class = {
    0 % 3
  } + {
    10 + 5
  }
  let new = "a"
  echo True
  echo case [], fn(v2) { class }(1) {
    [b, x, ..], _ -> {
      fn(v3) { 0 }("res")
    } % 1
    [1, 6, ..], 7 -> case <<"b":utf8>>, class + class {
      <<_:utf8>>, _ -> 1 * 4
      _, 7 -> fn(v4) { class }(100)
      v5, _ -> fn(v6) { 10 }("")
    }
    [_], _ -> class
    v7, v8 -> {
      v8 - v8
    } * 0
  }
  echo case Cv0("abc", ""), Error("data", 3) {
    Error(_, 7) as whole, _ -> "ab"
    Error("a", _), _ -> {
      new <> ""
    } <> {
      "constructor" <> "ab"
    }
    _, _ -> "b"
  }
  echo {
    {
      fn(v9) { 1 }("constructor")
    } + 4
  } - {
    case False, fn(v10, v11) { #("b", 0.5) }(0.1, 100.0) {
      v12, #("bc", 1.0) as whole if v12 && !v12 -> class
      acc, #(_, _) -> 5
      _, #("b" as whole, _) -> class * 2
      _, _ -> class % 5
    }
  }
}
