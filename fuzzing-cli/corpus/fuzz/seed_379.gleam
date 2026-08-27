pub const pi_value: String = "data"
pub const golden_value: String = "ab"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, l: String, v0: List(Int)) -> List(Int) {
[100]
}

fn f1(prototype: Float, m: #(List(Int), Float), acc: String) -> String {
case {
      let acc = prototype
      let item = [2, 1]
      ""
    }, acc <> acc {
    "x", v1 -> case True {
      _ | True -> fn(v2) { v1 }(True)
      item -> "a"
    }
    "abc" <> _, length -> "b" <> {
      {
        let acc = ""
        length
      }
    }
    "res", "a" -> fn(v3) { acc <> acc }(True)
    _, _ -> case acc <> "res", #([5, 4], False) {
      _, #([], this_) if this_ -> "res"
      "ab" <> _, #([a], True as whole) -> acc <> "constructor"
      "b", #([_] as whole, False) -> ""
      v4, v5 -> "data"
    }
  }
}

fn constructor(v6: Int, class: Bool) -> String {
"b"
}

pub fn main() {
  echo case {
      let y = 3.14
      let pair = 10.0
      pair
    } {
    0.5 -> {
      let acc = 2
      10 |> spin(acc % 5)
    }
    1.5 -> {
      4 |> spin(spin(0, 100))
    } % 3
    b -> 42
  }
  echo case #([3, 1], "bc") {
    #([], "constructor") -> case golden_value, {
        let pi_value = [1, 4]
        let class = [100]
        golden_value
      } {
      "" <> _, "" <> _ -> 1.5
      golden_value, "res" if golden_value != "b" && golden_value != "" -> 2.0
      _, v7 -> fn(v8, v9) { v9 }("constructor", 100.0)
    }
    #([4, ..rest], pi_value) if pi_value == "constructor" -> 3.14
    item -> {
      {
        2.0
      } +. {
        2.0
      }
    } -. {
      1.0
    }
  }
  echo {
    case {
        0.1
      } != {
        0.1
      } {
      a -> 2 - 0
      a -> 5
      golden_value -> fn(v10, v11) { 10 }("constructor", "b")
    }
  } != 2
  echo [4, 42]
}
