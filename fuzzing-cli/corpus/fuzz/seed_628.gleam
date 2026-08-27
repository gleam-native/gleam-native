pub const tag_value: Bool = False
pub const golden_value: Float = 100.0
pub const seed_value: String = "constructor"

fn f0(m: String, v0: List(Int), v1: #(List(Int), String)) -> List(Int) {
case fn(v2) { #([2], 1.0) }(True) {
    inner -> fn(v3, v4) { {
      let length = "ab"
      v0
    } }("data", 1.0)
    #([_], 1.5) -> {
      let arguments = 3
      let z = arguments
      {
        let z = True
        v0
      }
    }
    constructor -> case False, 5 {
      True, s -> v0
      False, 3 -> v0
      v5, _ -> v0
    }
  }
}

pub fn main() {
  let rest = case f0("constructor", [], #([], "res")), fn(v6) { [42] }(0.0) {
    [_, ..rest], [2, ..tail] -> seed_value
    [x], [a, 5, ..] -> seed_value
    v7, v8 -> seed_value
  }
  echo {
    3.14
  } >=. {
    golden_value *. {
      {
        let seed_value = [3, 5]
        let golden_value = golden_value
        golden_value
      }
    }
  }
  echo case golden_value <=. golden_value, seed_value <> rest {
    False, "abc" -> []
    _, "x" -> case 2 {
      3 -> []
      7 | 5 -> [42]
      a -> [100]
    }
    _, arguments -> case <<2:8, "bc":utf8>> {
      <<42:8>> -> [4, 7]
      _ -> f0(rest, [2, 10], #([42], "abc"))
    }
  }
  echo case golden_value, {
      let constructor = False
      let tag_value = rest
      4
    } {
    100.0, 6 as whole -> case seed_value <> "x", 100 - 10 {
      _, 5 -> "res" <> "res"
      "constructor", 6 -> "abc" <> rest
      _, _ -> "constructor" <> "x"
    }
    0.5, 1 -> case #(False, []) {
      #(_, [a]) -> rest
      #(False, [h, ..rest]) if h == 7 && h % 2 == 0 -> fn(v9) { seed_value }(5)
      #(acc, [9, ..rest]) -> {
        let golden_value = "constructor"
        golden_value
      }
      _ -> "b" <> rest
    }
    0.5, rest -> {
      "res" <> seed_value
    } <> "bc"
    v10, _ -> rest
  }
  echo {
    case 100 + 0 {
      inner -> inner
      l -> {
        let pair = l
        l
      }
      rest -> 10
    }
  } - {
    {
      fn(v11) { 0 }(True)
    } + 42
  }
}
