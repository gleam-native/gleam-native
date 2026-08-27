pub const golden_value: String = "bc"
pub const seed_value: Bool = False

pub type V0 {
  Record(value: String, inner: Int)
}

fn f0(l: Float, v1: String, v2: Int) -> List(Int) {
[4, 42]
}

fn extends(prototype: V0, v3: V0) -> List(Int) {
[]
}

fn class(v4: List(Int), v5: String) -> String {
case Record("b", 4), fn(v6, v7) { "abc" }("ab", "ab") {
    Record(z, _), "ab" <> _ -> "abc"
    Record(_, v8), "x" <> _ -> v5
    _, v9 -> {
      "a" <> v9
    } <> {
      {
        let y = v9
        y
      }
    }
  }
}

pub fn main() {
  echo {
    extends(Record("abc", 4), Record("a", 4)) |> class(golden_value <> golden_value)
  } == {
    fn(v10) { {
      let this_ = [7, 10]
      let z = golden_value
      golden_value
    } }(False)
  }
  echo f0(case 5 {
    v11 -> {
      3.14
    } -. {
      3.14
    }
    _ | 8 -> 0.1
    0 -> 0.0
  }, {
    "b" <> golden_value
  } <> {
    [1] |> class(golden_value)
  }, 2)
  echo {
    case golden_value {
      _ -> 10.0
      _ -> 2.0
    }
  } +. {
    0.0
  }
}
