pub const seed_value: Float = 2.0

pub type Map {
  Record
  Cv0(value: String, inner: List(Int))
  Cv1(value: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(item: List(Int)) -> List(Int) {
[0, 1]
}

fn f1(pair: String, new: Int, n: #(String, List(Int))) -> Bool {
case fn(v2) { 2 }("res"), Cv1(4) {
    _, v3 -> fn(v4, v5) { {
      let item = 0
      let n = v5
      True
    } }("ab", 100)
    7 as whole, Cv1(1) if whole % 2 == 0 && whole <= 0 -> case Cv0("", [3, 2]) {
      Cv0("res" <> _, [constructor, _, ..]) if constructor <= 2 -> True
      Cv1(1) -> True
      Cv0(a, _) -> False
      _ -> fn(v6, v7) { True }(True, False)
    }
    v8, Cv1(2 as whole) -> True
  }
}

fn f2(v9: Bool, v10: Bool, v11: #(Bool, List(Int))) -> Int {
case False {
    False -> case Record {
      Cv1(inner) -> 5
      Cv1(b) -> spin(100, 3)
      Record -> 5
      v12 -> 1 + 3
    }
    True | False -> {
      1 * 0
    } - {
      10 |> spin(4)
    }
    True -> 5 - {
      1 * 5
    }
  }
}

pub fn main() {
  echo {
    fn(v13, v14) { {
      let prototype = 4
      let acc = v13
      []
    } }(True, "constructor")
  } |> f0()
  echo "data"
  echo case "x" {
    "x" <> rest -> {
      {
        100.0
      } /. {
        3.14
      }
    } -. {
      {
        1.0
      } *. seed_value
    }
    inner -> case "b" {
      _ -> {
        let inner = seed_value
        let seed_value = "abc"
        inner
      }
      a | "res" <> a -> seed_value
      item -> seed_value
    }
    "x" <> _ -> case #(7, 3), 2.0 {
      #(_, 8), _ -> seed_value
      #(_, _), 3.14 -> 0.5
      #(_, 2), seed_value -> seed_value -. seed_value
      _, _ -> {
        1.0
      } -. seed_value
    }
  }
  echo {
    case 0 {
      3 -> fn(v15, v16) { "constructor" }("", "ab")
      _ -> "ab" <> "b"
    }
  } <> {
    {
      let seed_value = f2(False, True, #(True, [4]))
      {
        let seed_value = 2.0
        "ab"
      }
    }
  }
}
