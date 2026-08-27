fn f0(m: String, self_: List(Int)) -> List(Int) {
[2, 5]
}

pub fn main() {
  echo case #("abc", True) {
    inner -> 10
    item -> 4
    #(v0, _) -> case "a" <> v0 {
      "ab" | "a" <> _ -> 3 + 5
      "data" | "constructor" <> _ -> 7
      v1 -> 3
    }
  }
  echo case #(False, 1.5), {
      let this_ = "bc"
      let prototype = False
      prototype
    } {
    #(True, 1.5), True -> {
      fn(v2, v3) { 1.0 }("ab", False)
    } +. {
      fn(v4) { 3.14 }("res")
    }
    #(_, _), True -> {
      {
        0.1
      } /. {
        10.0
      }
    } -. {
      0.1
    }
    #(_, _), False -> case "abc", #(10, True) {
      "abc", #(2, True as whole) if whole -> 0.5
      self_, #(9, _) if self_ == "bc" || self_ != "" -> {
        3.14
      } +. {
        0.25
      }
      "" <> _, #(6, True) -> {
        let delete = 0.25
        10.0
      }
      _, _ -> {
        10.0
      } +. {
        1.0
      }
    }
    _, _ -> 100.0
  }
  echo case "x" <> "", 4 + 1 {
    "res", 4 -> 7 + {
      3 % 7
    }
    length, 2 -> 42
    _, v5 -> {
      let default = {
        let v5 = True
        5
      }
      5 % 5
    }
  }
  echo True
}
