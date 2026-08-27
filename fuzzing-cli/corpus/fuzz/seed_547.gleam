pub const pi_value: Float = 100.0

pub type Record {
  Cv0(value: String, inner: String)
}

pub type Map {
  Cv1(Int, value: List(Int))
  Cv2
  Cv3(Float, value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v4: Int) -> List(Int) {
[]
}

fn f1(v5: Bool, n: Int) -> String {
case "data", Cv2 {
    _, Cv2 -> {
      "" <> "b"
    } <> {
      {
        let m = []
        "res"
      }
    }
    _, Cv3(v5, "b" as whole) if v5 >=. 1.5 -> case whole, 7 {
      _, _ -> "a"
      _, 6 -> "bc"
    }
    "x", Cv2 -> "ab"
    _, _ -> fn(v6) { "" <> "bc" }(True)
  }
}

fn f2(v7: Int, v8: String) -> List(Int) {
case 10 |> f0(), v7 <= v7 {
    [], True -> case v7 - 10, Cv0("x", "constructor") {
      5, Cv0(v9, "res" <> _) as whole if v9 == "res" -> [100]
      0, Cv0("res", "x") -> fn(v10, v11) { [] }(0.25, "a")
      7 as whole, Cv0("constructor" <> rest, "constructor" as it) -> {
        let s = rest
        let m = 1.5
        []
      }
      _, _ -> [7, 5]
    }
    [v7], _ -> case v8 {
      _ -> {
        let rest = 5
        let n = []
        n
      }
      constructor | "data" <> constructor -> [42]
    }
    _, _ -> f0(0 - v7)
  }
}

pub fn main() {
  let new = case fn(v12, v13) { True }(2.0, 1), Cv3(0.1, "bc") {
    True, Cv2 -> False || False
    _, Cv2 -> pi_value >=. {
      1.0
    }
    v14, _ -> v14
  }
  echo fn(v15) { case {
      let pi_value = new
      3
    } {
    constructor -> {
      let s = "data"
      let s = []
      "bc"
    }
    item -> "a"
  } }(True)
  echo {
    {
      "abc" <> "constructor"
    } <> {
      "" <> "ab"
    }
  } <> {
    {
      fn(v16, v17) { True }("x", 100)
    } |> f1(42)
  }
  echo case fn(v18, v19) { 7 }(False, 1) {
    constructor -> case "bc" {
      a -> {
        let new = 0.5
        let s = 10
        3.14
      }
      "x" -> pi_value -. {
        0.0
      }
      "data" <> _ as whole -> pi_value +. pi_value
    }
    4 | 5 -> pi_value
    a -> 1.5
  }
  echo "x"
}
