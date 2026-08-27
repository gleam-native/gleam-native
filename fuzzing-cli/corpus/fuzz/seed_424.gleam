fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, v1: List(Int)) -> String {
case v0, fn(v2) { v1 }(False) {
    _, [] -> case True {
      _ -> "abc" <> "data"
      a -> "ab"
      v3 -> "bc" <> "ab"
    }
    5, [5, _, ..] -> case 10.0 {
      v1 -> "data"
      _ | 1.0 -> {
        let v1 = True
        let constructor = v1
        "res"
      }
      b -> "ab" <> "data"
    }
    _, _ -> case "b", "abc" {
      _, "b" <> rest -> rest <> rest
      "constructor" as whole, "b" -> "x"
      _, v4 -> v4 <> v4
    }
  }
}

fn new(v5: Float) -> Float {
v5
}

pub fn main() {
  echo {
    case 3.14 {
      inner -> 0
      10.0 -> 1 - 10
    }
  } |> f0(0 + 1, [4, 7])
  echo []
  echo fn(v6, v7) { case [2, 1] {
    [3] -> []
    [] -> []
    _ -> [4]
  } }("data", 0)
  echo {
    {
      let delete = "a"
      delete <> "a"
    }
  } <> {
    "data" <> f0(1, 7, [0])
  }
}
