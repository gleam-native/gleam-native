pub const golden_value: String = "abc"
pub const seed_value: String = "b"

pub type V0 {
  None(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
}

pub type V3 {
  Ok
}

fn f0(value: #(String, Int)) -> Bool {
case "x", {
      let length = False
      0
    } {
    "data" as whole, value -> False
    "b" <> _, _ -> True
    class, value -> fn(v4, v5) { {
      100.0
    } != {
      2.0
    } }(True, "res")
  }
}

fn class(delete: Bool, acc: List(Int), default: V3) -> String {
case <<"x":utf8>> {
    <<_:big-signed-8, _:little-signed-8, _:8>> -> case delete, Cv2 {
      True, Cv2 -> "abc"
      _, Cv2 as whole -> {
        let l = [10]
        "abc"
      }
      _, Cv2 -> fn(v6) { "" }(True)
      v7, _ -> "bc"
    }
    <<"res":utf8, _:8>> -> "b" <> "res"
    _ -> "abc"
  }
}

pub fn main() {
  let seed_value = {
    let golden_value = []
    let rest = 10.0
    [100, 2]
  }
  echo case {
      let golden_value = 1.5
      Ok
    }, <<"":utf8, 4:8>> {
    Ok, <<5:8, _:4>> -> True
    Ok, <<7:8>> -> True
    Ok, <<_:utf8>> as whole -> f0(#("data", 42))
    _, v8 -> False
  }
  echo 1.0
  echo False
}
