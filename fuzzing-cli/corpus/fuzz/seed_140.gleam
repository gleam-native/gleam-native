pub const pi_value: Int = 2
pub const tag_value: Float = 3.14
pub const golden_value: Int = 0

pub type V0 {
  None(value: String, inner: String)
  Cv1
  Cv2(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(default: Int, v3: Int, v4: #(String, Int)) -> Int {
{
    5 + {
      4 - 5
    }
  } + {
    case <<"b":utf8>> {
      <<_:1>> -> default - 2
      _ -> v3 - 100
    }
  }
}

fn f1(v5: Float, v6: Float, v7: #(Int, String)) -> Float {
case "res" <> "data", v5 {
    v6, 0.0 if v6 != "a" && v6 == "x" -> {
      v5 +. v5
    } -. {
      v5 -. v5
    }
    "abc", 10.0 -> v5
    v8, v9 -> v6
  }
}

pub fn main() {
  let pair = 10
  echo pi_value + pair
  echo case None("abc", "") {
    Cv1 -> []
    None(b, _) -> [4]
    _ -> {
      let tag_value = "constructor"
      []
    }
  }
  echo [2]
}
