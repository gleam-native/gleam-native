pub const seed_value: Float = 0.25
pub const tag_value: String = ""

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: String)
}

pub type V3 {
  Cv4(List(Int))
}

fn f0(v5: String) -> Bool {
case <<7:8, "":utf8>> {
    <<"bc":utf8, _:big-signed-8, "abc":utf8>> -> case Cv4([3, 42]) {
      default -> False
      _ -> True || True
    }
    _ -> {
      fn(v6, v7) { False }(4, 1)
    } || {
      {
        let z = 1.0
        let n = v5
        False
      }
    }
  }
}

pub fn main() {
  let v = case tag_value {
    "data" <> rest -> {
      let pair = 42
      3
    }
    "abc" <> rest -> 7
    "" <> rest -> 2
    v8 -> 7
  }
  let seed_value = [5, 5]
  echo {
    {
      let arguments = 1.5
      let z = v - v
      fn(v9) { tag_value }(False)
    }
  } <> {
    case fn(v10) { 10 }(True) {
      inner -> fn(v11) { "" }(4)
      _ -> tag_value
      _ | 5 -> "x"
    }
  }
  echo tag_value
  echo {
    let seed_value = case <<"bc":utf8, "res":utf8, "a":utf8>> {
      <<4:1>> -> v + v
      <<"b":utf8, _:8>> -> 1
      v12 -> {
        let tag_value = tag_value
        let value = 10.0
        v
      }
    }
    case "abc" {
      constructor -> fn(v13) { "bc" }(0.25)
      "abc" -> tag_value <> tag_value
      "res" <> _ -> tag_value <> tag_value
    }
  }
}
