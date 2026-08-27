pub const seed_value: Float = 1.5

pub type Promise {
  Cv0(value: String, inner: Int)
  Error(value: Int, inner: String)
}

fn static(v1: Int) -> Int {
v1 + 100
}

fn f1(class: Float, v2: String) -> Float {
case "x", {
      let arguments = [0]
      let self_ = ""
      1
    } {
    "res", _ -> 1.5
    "data", 1 -> {
      0.0
    } -. {
      class +. {
        10.0
      }
    }
    "b" <> _ as whole, _ -> fn(v3, v4) { {
      1.5
    } *. class }("", True)
    v5, v6 -> case fn(v7, v8) { v5 }(5, False) {
      "res" | "bc" -> class -. {
        0.5
      }
      v6 -> class
    }
  }
}

pub fn main() {
  let self_ = fn(v9, v10) { !False }(False, False)
  echo case 10 {
    v11 -> f1(0.0, "abc") /. {
      0.5
    }
    b -> seed_value
    4 -> seed_value
  }
  echo fn(v12) { case "bc" <> "res" {
    "ab" | "a" -> "abc"
    "bc" -> "" <> "x"
    _ -> "data"
  } }(3)
  echo {
    case 4, "x" {
      _, self_ -> seed_value
      _, "ab" -> f1(0.25, "ab")
    }
  } *. {
    case {
        let m = []
        Cv0("x", 2)
      } {
      Error(_, "abc" as whole) as it if whole == "a" -> fn(v13) { 2.0 }("abc")
      Cv0("a" <> rest as whole, _) -> 0.5
      _ -> fn(v14, v15) { 0.1 }(True, "")
    }
  }
}
