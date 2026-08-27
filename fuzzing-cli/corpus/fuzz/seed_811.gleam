pub type V0 {
  Record(value: String, inner: Float)
  Cv1
}

pub type Number {
  Cv2(Bool, value: String)
}

fn delete(v3: #(Float, String)) -> String {
fn(v4, v5) { "a" <> {
    fn(v6, v7) { v6 }("bc", 1)
  } }(1, 0.5)
}

fn yield(self_: Float, v8: List(Int)) -> Int {
case "a" <> "ab" {
    _ | "b" <> _ -> case Cv2(False, "ab"), False {
      Cv2(v9, "constructor" <> _ as whole), delete if whole == "b" -> 7
      _, _ -> fn(v10, v11) { 7 }("x", "bc")
    }
    "res" | "x" -> 5
    "ab" as whole -> 1
  }
}

pub fn main() {
  echo 1.5
  echo case 7 {
    new -> case "b" <> "abc" {
      "data" -> new + 42
      _ -> new - new
    }
    5 -> {
      {
        let value = 2.0
        7
      }
    } - {
      {
        let constructor = True
        let constructor = True
        2
      }
    }
    constructor -> case Cv2(False, ""), <<42:8>> {
      Cv2(_, _), <<0:8>> -> 42 + 42
      _, <<2:8, 4:8>> as whole -> constructor - constructor
      _, _ -> yield(0.0, [10, 2])
    }
  }
  echo {
    case 4 {
      item -> {
        let value = True
        100.0
      }
      v -> {
        0.5
      } *. {
        100.0
      }
      a -> {
        0.25
      } -. {
        3.14
      }
    }
  } /. {
    2.0
  }
}
