pub const tag_value: Int = 10
pub const seed_value: Float = 2.0
pub const golden_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

pub type Symbol {
  Cv3(value: Int)
  Cv4(String, value: String)
  Cv5(value: List(Int), inner: String)
}

fn f0(v6: V0, v7: String, n: Int) -> Float {
case n * n, Cv2 {
    _, Cv2 -> 0.25
    7, Cv1([], 1 as whole) -> 0.1
    v8, v9 -> 1.0
  }
}

fn f1(v10: Bool, m: Int) -> List(Int) {
{
    let m = True
    case fn(v11) { True }(3), "a" {
      v12, _ -> [42, 10]
      v13, "abc" -> {
        let l = 2.0
        let prototype = 0.0
        [1, 1]
      }
    }
  }
}

pub fn main() {
  let pair = fn(v14) { False || True }(1.0)
  echo {
    case <<"b":utf8, "bc":utf8, "x":utf8>> {
      <<_:utf8, "b":utf8>> -> "x" <> "res"
      <<_:utf8>> -> "abc" <> "a"
      _ -> "x"
    }
  } <> {
    {
      fn(v15, v16) { v15 }("constructor", "bc")
    } <> "constructor"
  }
  echo {
    let delete = "" <> "a"
    Cv2 |> f0("", {
      let class = pair
      let rest = class
      tag_value
    })
  }
}
