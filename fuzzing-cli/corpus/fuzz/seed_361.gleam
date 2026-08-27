pub const tag_value: Bool = True

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Float, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(m: Int, l: #(String, String)) -> Bool {
{
    case <<"":utf8, "bc":utf8, "b":utf8>> {
      <<_:utf8>> -> False
      _ -> False
    }
  } || {
    !{
      {
        let m = True
        let acc = 0
        True
      }
    }
  }
}

fn static(v3: V0, class: String) -> Float {
3.14
}

fn f2(pair: Bool) -> List(Int) {
case "constructor" <> "a" {
    item -> []
    "a" as whole -> case fn(v4) { 100.0 }(True) {
      constructor -> [1]
      3.14 -> fn(v5) { [0, 1] }(10.0)
    }
    v6 | "res" <> v6 -> [5]
  }
}

pub fn main() {
  let length = "res"
  echo case length <> length {
    "a" -> "data"
    _ -> "b" <> "res"
    a -> case a {
      constructor -> "data"
      "" <> inner -> {
        let a = "bc"
        let z = 0.25
        "b"
      }
      _ -> {
        let tag_value = 0.5
        "b"
      }
    }
  }
  echo length <> "res"
}
