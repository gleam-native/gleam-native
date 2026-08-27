pub const pi_value: Float = 0.25

pub type Number {
  Cv0(value: String, inner: Float)
  Cv1
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Number) -> List(Int) {
case #([], True) {
    #([_, v2, ..], True) if v2 == 6 || v2 > 4 -> case fn(v3, v4) { Cv0("res", 1.5) }(3.14, 0.0) {
      Cv0("a" <> rest as whole, 0.0) if rest != "data" && rest == "abc" -> []
      Cv0("" <> rest, 1.5) -> [5]
      Cv0(_, b) -> fn(v5) { [] }("x")
      v6 -> [42]
    }
    #([4], _) | #([_, _, ..], False) -> {
      let n = "ab"
      let constructor = True && False
      []
    }
    v7 -> fn(v8, v9) { [5] }(0, "a")
  }
}

fn f1(v10: Bool, v11: Int, n: List(Int)) -> Int {
case v11 {
    v12 -> 10
    inner -> 4
  }
}

pub fn main() {
  let v = {
    let l = 4
    fn(v13) { "abc" }(3)
  }
  echo case True {
    b -> [3]
    True -> [7, 42]
    _ -> [2, 0]
  }
  echo {
    {
      fn(v14) { "res" }(7)
    } <> "ab"
  } <> "ab"
  echo case 7 * 4 {
    pair -> [2]
    inner -> case 3 {
      2 as whole -> [7]
      _ | 4 -> {
        let inner = inner
        [4]
      }
      v15 -> fn(v16, v17) { [3] }(2.0, 0)
    }
    item -> [0]
  }
  echo 3 + {
    case 5, Cv0("x", 0.5) {
      _, Cv1 -> 42
      3, Cv1 -> 0
      _, Cv1 -> 42 |> spin(5 + 1)
      _, _ -> 42
    }
  }
}
