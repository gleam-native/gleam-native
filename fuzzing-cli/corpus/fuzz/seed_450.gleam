pub const limit_value: Bool = True
pub const seed_value: String = "x"

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(constructor: Bool) -> Int {
walk(case "ab" {
    "ab" as whole if whole != "abc" -> [0]
    "a" | "x" -> []
    "bc" <> constructor -> []
    _ -> []
  }, case "ab" <> "", "" <> "a" {
    "data", "a" <> rest if rest == "b" -> [3] |> walk(0)
    "ab" as whole, "b" -> 2
    _, _ -> {
      let z = 2
      let self_ = constructor
      z
    }
  })
}

fn constructor(v3: Int, v: String, v4: Int) -> Float {
3.14
}

pub fn main() {
  let limit_value = case <<"b":utf8>> {
    <<"data":utf8, 7:8>> -> seed_value
    <<_:utf8>> -> seed_value <> seed_value
    v5 -> "b"
  }
  echo constructor(case walk([], 4), fn(v6) { v6 }(2) {
    _, _ -> walk([5, 4], 4)
    2, 1 as whole -> 7 + 2
    8, _ -> 4
  }, case {
      let constructor = 10
      let length = 0.0
      0
    } {
    seed_value -> {
      let limit_value = []
      let delete = 10
      "ab"
    }
    inner -> "x"
  }, {
    5 * 42
  } + {
    7 + 2
  })
  echo case constructor(0, "x", 0), {
      let y = 7
      let pair = 42
      limit_value
    } {
    v7, _ -> {
      let limit_value = fn(v8) { True }("constructor")
      let limit_value = {
        let self_ = 2
        100
      }
      fn(v9, v10) { [100] }(True, "b")
    }
    v11, _ -> case {
        let seed_value = 5
        seed_value
      } {
      _ | 2 -> [1, 2]
      a -> [42, 42]
    }
  }
}
