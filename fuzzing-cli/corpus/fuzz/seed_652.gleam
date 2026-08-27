pub const tag_value: Float = 0.0

pub type V0 {
  Ok(value: String, inner: String)
  Cv1(String)
  Cv2(List(Int), String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(default: Int) -> Bool {
True
}

fn class(length: Bool, v3: List(Int)) -> Int {
5 - {
    7 - 5
  }
}

fn yield(length: Float, v4: Float, x: Float) -> Bool {
False
}

pub fn main() {
  let s = tag_value
  echo case 2.0 {
    v5 -> [0]
    item -> [2, 7]
  }
  echo case <<"abc":utf8, 42:4>> {
    <<4:8>> -> {
      tag_value -. {
        3.14
      }
    } /. {
      2.0
    }
    <<"ab":utf8>> -> {
      tag_value *. {
        1.0
      }
    } /. {
      0.5
    }
    <<"bc":utf8, delete:4>> -> case {
        let arguments = [1]
        let z = tag_value
        2
      } {
      length -> s +. {
        1.0
      }
      9 | 6 -> tag_value /. {
        3.14
      }
      3 -> s *. s
    }
    v6 -> tag_value
  }
  echo 1
  echo case "x" <> "abc" {
    item | "res" <> item -> fn(v7) { 10 }(False)
    "x" <> _ -> class(False, [42, 100]) * {
      42 + 100
    }
    _ -> case [] {
      [h, ..rest] -> {
        let this_ = rest
        let rest = True
        4
      }
      [1] -> 100 - 5
      [1, s, ..] as whole -> 10
      _ -> 10
    }
  }
}
