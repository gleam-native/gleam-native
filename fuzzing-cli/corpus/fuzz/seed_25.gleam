pub const pi_value: Float = 10.0
pub const tag_value: Int = 5
pub const euler_value: Float = 1.5

pub type V0 {
  Ok(value: String, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(arguments: V0) -> String {
"constructor"
}

pub fn main() {
  let default = case {
      0.25
    } /. {
      1.0
    }, 1 % 5 {
    0.0 as whole, 7 -> []
    0.1, _ -> [7]
    _, v1 -> [10, 7]
  }
  let pair = "x" <> {
    {
      let arguments = True
      "abc"
    }
  }
  echo {
    let z = default
    case fn(v2, v3) { Ok("data", 2) }(4, False) {
      item -> 2 - tag_value
      Ok(_, _) -> 2
      Ok(pi_value, 0 as whole) -> {
        let rest = True
        let delete = rest
        0
      }
    }
  }
  echo case "bc" {
    "x" <> rest -> {
      {
        2.0
      } +. euler_value
    } -. {
      0.1
    }
    "constructor" <> rest if rest != "res" && rest != "x" -> case False {
      inner -> 0.0
      inner -> 0.5
    }
    "x" <> item | "a" <> item -> pi_value
    _ -> 0.1
  }
  echo "b"
  echo case <<0:8>> {
    <<5:16>> -> []
    <<5:16>> -> {
      let default = "bc"
      let s = True && False
      []
    }
    _ -> {
      let s = f0(Ok("res", 42))
      let value = fn(v4, v5) { v5 }(10, "b")
      {
        let v = s
        let tag_value = s
        default
      }
    }
  }
}
