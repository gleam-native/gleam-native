pub type V0 {
  None(value: String, inner: Float)
  Some(List(Int), Int)
}

fn yield(m: Bool, v1: Bool, v2: Int) -> Bool {
{
    let pair = case 1 {
      inner -> fn(v3, v4) { "bc" }(True, 1.5)
      item -> ""
      a -> "abc"
    }
    False
  }
}

fn f1(self_: Bool, v5: Float) -> Int {
case v5 {
    item -> case 10.0 {
      constructor -> fn(v6, v7) { 7 }(1.5, "a")
      0.1 | 1.5 -> 2
      v8 -> 1
    }
    inner -> 42
    _ -> case "bc" {
      "x" as whole -> 1
      "constructor" -> fn(v9) { 5 }(False)
      _ -> {
        let length = False
        2
      }
    }
  }
}

fn f2(v10: Float) -> List(Int) {
case "ab" <> "data" {
    "constructor" <> rest -> case 0.25, {
        let v10 = v10
        ""
      } {
      1.0, rest -> [0]
      0.25, "res" <> rest -> [2]
      _, _ -> []
    }
    item -> [3]
  }
}

pub fn main() {
  let constructor = case [5], 100 {
    [_], 9 -> 0.25
    [8, ..rest], 3 -> 2.0
    [a, ..rest], _ -> 100.0
    _, _ -> fn(v11, v12) { 0.5 }(10, "a")
  }
  let length = 2
  echo case fn(v13) { [] }(7) {
    [_, _, ..] -> case False {
      _ -> {
        3.14
      } |> f2()
      item -> f2(constructor)
      inner -> f2(0.0)
    }
    [5, _, ..] as whole -> case None("bc", 1.5), length * 10 {
      None("abc", 2.0 as whole), 9 -> [100, 10]
      _, 1 -> [1]
      _, _ -> [3, 10]
    }
    [5, ..rest] as whole -> whole
    _ -> case 0 {
      5 -> []
      4 -> [4]
      0 | 8 -> constructor |> f2()
      _ -> {
        let this_ = True
        let y = "a"
        [7]
      }
    }
  }
  echo constructor |> f2()
  echo ""
}
