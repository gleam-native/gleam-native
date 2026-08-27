pub type Promise {
  Cv0(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
  Cv3(value: Float, inner: String)
}

fn extends(v4: Int, length: String) -> Float {
case {
      let v4 = 3
      10.0
    }, "x" <> length {
    0.25, v5 -> 0.1
    _, "constructor" <> rest -> 0.25
    0.1, "bc" -> {
      {
        3.14
      } -. {
        0.5
      }
    } +. {
      0.1
    }
    v6, v7 -> 2.0
  }
}

pub fn main() {
  echo {
    case extends(10, "bc"), Cv0("b", [10, 4]) {
      _, Cv0(_, [_, h, ..] as whole) as it -> 4 + h
      _, Cv0("ab", []) as whole -> 7
      _, v8 -> 5 + 5
    }
  } % 4
  echo {
    case Cv0("res", [3, 7]), fn(v9) { 3 }("bc") {
      Cv0("res", [9, ..rest]) as whole, 8 -> fn(v10, v11) { 10.0 }("constructor", "res")
      Cv0(s, [h, ..rest]) as whole, _ if s != "constructor" -> {
        let y = False
        let n = 100
        1.0
      }
      Cv0(_, [x, ..rest]) as whole, 0 -> 1.0
      _, _ -> 0.25
    }
  } -. {
    3.14
  }
  echo case "a" {
    _ -> case "x", {
        let value = True
        let constructor = []
        7
      } {
      "b", 9 -> 2 == 42
      "" <> rest, this_ -> True
      v12, _ -> True
    }
    _ -> False
    "res" -> case Cv2 {
      Cv3(_, item) -> fn(v13) { False }("bc")
      Cv3(_, inner) -> inner != "b"
      Cv3(_, "constructor" <> _) -> {
        3.14
      } <. {
        0.1
      }
      _ -> True
    }
  }
}
