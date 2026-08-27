pub const tag_value: Int = 0
pub const golden_value: Float = 0.0

fn constructor(constructor: Int, z: Int, prototype: Bool) -> Int {
case <<1:8>> {
    <<"a":utf8, new:4, _:big-unsigned-8>> if new > 4 || new <= 6 -> 1
    _ -> case "constructor", fn(v0) { "abc" }(False) {
      "x", "res" -> z % 1
      "bc", _ -> constructor - z
      _, _ -> {
        let s = 1.0
        let z = s
        42
      }
    }
  }
}

pub fn main() {
  let golden_value = {
    let default = 0.1
    let z = 10.0
    golden_value
  }
  echo case "x" <> "" {
    "" <> rest | "a" <> rest -> "abc"
    item | "constructor" <> item -> {
      "" <> item
    } <> {
      fn(v1, v2) { item }(7, 1.5)
    }
    "data" | "constructor" <> _ -> {
      "ab" <> ""
    } <> "a"
  }
  echo True
  echo case tag_value - tag_value, fn(v3) { [] }(True) {
    1, [4, constructor, ..] as whole if constructor == 5 -> {
      let prototype = True
      let x = golden_value
      "ab" <> "abc"
    }
    v4, [tag_value] -> "b" <> {
      fn(v5, v6) { "ab" }(True, "b")
    }
    _, [] -> "constructor" <> "b"
    v7, v8 -> "abc" <> {
      {
        let rest = ""
        let l = 2
        "x"
      }
    }
  }
  echo {
    let default = case {
        let v = []
        let arguments = "a"
        "data"
      }, 5 {
      _, 4 as whole if whole % 2 == 0 -> fn(v9, v10) { [10, 0] }(2, "b")
      "abc", 3 -> []
      "bc", 1 -> [2]
      _, _ -> [4, 7]
    }
    let m = golden_value
    case tag_value % 7 {
      v11 -> {
        let s = "res"
        let constructor = default
        "constructor"
      }
      a -> "bc" <> "constructor"
      inner -> fn(v12, v13) { "data" }(100.0, 10.0)
    }
  }
}
