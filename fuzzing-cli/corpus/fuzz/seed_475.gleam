pub const golden_value: String = "data"
pub const tag_value: Float = 0.25

pub type Promise {
  Cv0(value: String, inner: List(Int))
  Error
}

pub type V1 {
  Cv2(List(Int), Int)
}

fn f0(v3: Bool, v4: String) -> List(Int) {
case 3 {
    _ -> case <<"x":utf8>>, "x" == "ab" {
      <<0:1>>, _ -> fn(v5, v6) { [7] }(100.0, "")
      <<_:utf8, "bc":utf8>> as whole, _ -> [7, 2]
      _, False -> fn(v7) { [] }(0.25)
      _, _ -> fn(v8, v9) { [] }(0.1, "x")
    }
    v4 -> [1, 10]
  }
}

pub fn main() {
  let x = {
    "" <> golden_value
  } <> "abc"
  echo 1
  echo case <<0:16>> {
    <<3:16>> as whole -> False
    <<_:utf8>> -> case f0(True, "x") {
      [7] -> {
        10.0
      } <=. {
        1.5
      }
      [x, ..rest] if x > 5 || x <= 0 -> True || True
      [_] -> {
        let x = 10
        let class = False
        True
      }
      _ -> tag_value <. {
        10.0
      }
    }
    _ -> {
      fn(v10) { tag_value }(False)
    } >=. {
      tag_value -. {
        0.5
      }
    }
  }
  echo fn(v11) { case {
      let acc = 4
      let golden_value = acc
      []
    }, 42 {
    [x, ..rest], _ if x <= 5 -> "data"
    [0, constructor, ..] as whole, 8 as it -> v11
    [3, 4, ..] as whole, 0 as it -> ""
    v12, _ -> {
      let x = 4
      v11
    }
  } }("a")
  echo {
    {
      1 - 0
    } - {
      0 - 7
    }
  } - {
    case x <> "res" {
      constructor -> {
        let constructor = 4
        2
      }
      "x" <> rest | "a" <> rest -> 42
      "abc" <> rest -> 4 - 7
    }
  }
}
