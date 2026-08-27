pub const limit_value: String = "bc"

fn default(constructor: Int, x: List(Int), delete: Bool) -> Bool {
case "a" <> "constructor" {
    "x" <> b if b != "ab" && b != "bc" -> delete && {
      {
        let delete = constructor
        False
      }
    }
    inner | "b" <> inner -> case "data", <<"x":utf8>> {
      "a", <<4:8>> -> delete
      "ab", <<10:1, _:big-signed-8>> -> {
        let this_ = inner
        True
      }
      "b" <> rest, _ -> True
      _, _ -> {
        let default = x
        False
      }
    }
    "data" | "constructor" <> _ -> {
      fn(v0, v1) { constructor }(42, 100)
    } > {
      3 - 3
    }
  }
}

fn f1(v2: #(String, String)) -> Int {
7 + {
    case {
        let length = "res"
        10.0
      } {
      0.1 -> 1
      v3 -> 2 * 10
    }
  }
}

pub fn main() {
  let delete = 42
  echo {
    2.0
  } != {
    {
      0.5
    } -. {
      {
        1.0
      } -. {
        0.1
      }
    }
  }
  echo limit_value <> {
    {
      let acc = [4]
      let s = limit_value <> "a"
      limit_value <> "res"
    }
  }
  echo {
    {
      let constructor = {
        3.14
      } +. {
        1.0
      }
      {
        let y = 7
        y
      }
    }
  } + {
    case limit_value, "res" <> "ab" {
      _, x -> delete
      _, "data" <> _ as whole if whole != "b" -> 7
      "constructor", v4 -> f1(#("b", "res"))
    }
  }
}
