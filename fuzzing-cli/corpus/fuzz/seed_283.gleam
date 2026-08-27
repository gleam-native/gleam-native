pub const limit_value: String = "data"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, v0: Float, v1: String) -> List(Int) {
case 0, 5 - 100 {
    v1, _ -> case {
        let length = "x"
        length
      }, fn(v2, v3) { [100, 4] }(False, 10) {
      constructor, [_, ..rest] if constructor == "b" && constructor == "" -> {
        let this_ = v0
        rest
      }
      "data" <> _, [] -> [10, 10]
      _, v4 -> v4
    }
    8, _ -> case [42] {
      [constructor, ..rest] -> rest
      [1, a, ..] -> [42, 10]
      [6] -> []
      _ -> []
    }
  }
}

fn f1(new: Int) -> Float {
case fn(v5) { "b" }(False) {
    "res" <> b -> fn(v6) { {
      0.5
    } /. {
      1.0
    } }(2.0)
    "bc" <> b -> 100.0
    _ -> case f0(False, 10.0, "constructor") {
      [9, ..rest] -> {
        0.1
      } +. {
        0.1
      }
      [b] -> {
        let b = b
        0.25
      }
      [_, h, ..] -> 0.5
      _ -> {
        0.5
      } +. {
        1.5
      }
    }
  }
}

fn yield(this_: String, v7: String) -> Float {
2.0
}

pub fn main() {
  let limit_value = case True {
    _ | True -> fn(v8) { [] }(True)
    a -> {
      let prototype = limit_value
      let prototype = 0
      [10]
    }
  }
  let n = fn(v9) { "a" == "a" }(False)
  echo limit_value
  echo {
    case "constructor" <> "b", "data" {
      "res", "constructor" as whole -> 1 - 1
      "abc" as whole, "x" as it if it != "a" -> 4 + 3
      this_, "bc" <> rest -> 3
      v10, _ -> 7
    }
  } <= {
    {
      {
        let x = "ab"
        let pair = False
        5
      }
    } - 5
  }
  echo case 100 {
    1 -> {
      let constructor = limit_value
      let rest = 1
      fn(v11, v12) { n }(1, "a")
    }
    4 | 9 -> n || {
      {
        1.5
      } != {
        0.25
      }
    }
    _ -> case spin(3, 7), #(False, 2) {
      limit_value, #(_, 0) -> True
      5, #(_, _) -> {
        let y = [3]
        let limit_value = n
        False
      }
      _, #(True, 9) -> False
      _, v13 -> fn(v14) { n }(0)
    }
  }
  echo fn(v15) { True |> f0(10.0, "b") }(1)
}
