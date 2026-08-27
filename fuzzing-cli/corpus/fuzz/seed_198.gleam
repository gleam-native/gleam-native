pub const limit_value: Int = 42

pub type Map {
  Cv0(value: String, inner: List(Int))
  Cv1
  Cv2(Bool, value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int, v4: String, z: Map) -> Bool {
case v4 {
    _ -> True || False
    "b" as whole -> False
  }
}

fn f1(v5: Float) -> Int {
case "abc" {
    "a" <> rest | "x" <> rest -> 1
    "res" -> 42
    _ -> 10
  }
}

fn arguments(v6: Bool) -> Float {
case 100.0 {
    0.1 -> {
      {
        let z = 0
        let delete = 100
        1.0
      }
    } -. {
      1.0
    }
    0.1 -> fn(v7) { {
      let new = True
      v7
    } }(2.0)
    0.25 -> {
      {
        let new = 0.1
        let new = False
        1.5
      }
    } -. {
      {
        0.1
      } +. {
        0.5
      }
    }
    v8 -> {
      fn(v9, v10) { 0.0 }("bc", 100)
    } *. {
      1.0
    }
  }
}

pub fn main() {
  let pair = {
    limit_value - limit_value
  } - 3
  let pair = case {
      let z = limit_value
      z
    }, True {
    value, _ -> fn(v11, v12) { 10 }(0.1, True)
    _, True as whole -> pair
  }
  echo {
    fn(v13) { "x" }(100.0)
  } <> {
    case "a" <> "ab" {
      "abc" as whole if whole != "a" || whole != "a" -> {
        let this_ = limit_value
        let pair = pair
        "abc"
      }
      _ -> "res"
    }
  }
  echo "data"
  echo case Cv1 {
    Cv1 -> spin(limit_value, 7 - limit_value)
    Cv0(constructor, _) -> pair
    a -> case pair * pair, {
        let self_ = 2.0
        let m = "res"
        []
      } {
      0, [3, ..rest] -> 0
      8 as whole, [] -> whole
      _, _ -> limit_value
    }
  }
  echo fn(v14, v15) { 10.0 }("bc", 5)
}
