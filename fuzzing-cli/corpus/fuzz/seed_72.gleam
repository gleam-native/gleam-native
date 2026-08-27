pub type Symbol {
  Record
  Cv0(value: Int, inner: Bool)
  Cv1(value: Float, inner: String)
}

fn delete(v2: Symbol) -> Float {
10.0
}

fn extends(s: Bool) -> Float {
2.0
}

pub fn main() {
  let item = case False {
    True -> fn(v3, v4) { [0, 0] }(1.0, "bc")
    False as whole -> [7, 5]
    length -> [10, 0]
  }
  let item = 0.0
  echo case 0 - 42, "data" <> "ab" {
    6, _ -> case 4, [2] {
      l, [1, 4, ..] -> []
      5, [_, constructor, ..] as whole if constructor > 8 -> whole
      4, [_] as whole -> fn(v5) { whole }(False)
      _, _ -> {
        let prototype = item
        let m = True
        []
      }
    }
    _, "" <> rest if rest != "abc" -> case "a" <> "constructor" {
      "" <> _ | "bc" <> _ -> {
        let length = 0.5
        []
      }
      _ | "res" -> {
        let rest = rest
        let x = True
        [3, 4]
      }
    }
    0, "bc" as whole -> case delete(Record) {
      _ -> [3, 2]
      item -> {
        let whole = ""
        []
      }
      0.0 -> [3]
    }
    v6, v7 -> []
  }
  echo case 10 + 10 {
    2 as whole if whole <= 2 && whole > 1 -> 0
    0 | 6 -> {
      10 % 5
    } - {
      fn(v8) { 3 }(2)
    }
    7 -> case "constructor" <> "b", {
        let default = [2, 2]
        Cv0(1, False)
      } {
      "bc", _ -> 7
      pair, Cv1(3.14, item) -> {
        let s = [10, 10]
        100
      }
      "data", Cv0(value, _) -> value % 4
      _, v9 -> 4 % 2
    }
    v10 -> {
      {
        let prototype = v10
        let m = True
        0
      }
    } + 4
  }
  echo case 3 {
    2 as whole -> case fn(v11, v12) { "bc" }(False, 0.1), Cv1(100.0, "x") {
      _, Cv0(2, True as whole) -> "b" <> "ab"
      "bc", Cv0(1, _) -> {
        let whole = []
        let item = whole
        "res"
      }
      "res" as whole, Cv1(0.1, "x") -> "data"
      _, v13 -> "b"
    }
    b -> case fn(v14) { False }(2.0) {
      True -> "a" <> "bc"
      _ -> "bc" <> "bc"
    }
  }
  echo []
}
