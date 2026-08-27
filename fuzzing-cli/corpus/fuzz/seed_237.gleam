pub const pi_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, m: Float, value: Int) -> Int {
value + 1
}

pub fn main() {
  let z = "bc"
  let acc = {
    1.0
  } -. {
    {
      10.0
    } -. {
      1.0
    }
  }
  echo case "data" <> "ab" {
    _ | "a" <> _ -> {
      z <> z
    } <> z
    "data" <> _ -> case fn(v0, v1) { z }(0.25, True) {
      "ab" -> z
      "bc" <> _ -> {
        let n = acc
        "ab"
      }
      v2 -> z <> v2
    }
    "" <> rest -> fn(v3) { {
      let this_ = 3.14
      rest
    } }(0)
  }
  echo {
    let this_ = case #([], [2, 5]), "b" {
      #([_, ..rest], [4]), "b" -> fn(v4, v5) { "bc" }(True, 1.0)
      #([_, 2, ..], []), "abc" <> rest -> rest <> rest
      v6, _ -> ""
    }
    let pi_value = case [], fn(v7, v8) { #(5, [3]) }(True, "") {
      [1, ..rest], #(5, []) -> "b" <> this_
      [9], #(1 as whole, [_, a, ..]) if a > 5 || a <= 9 -> fn(v9, v10) { this_ }(3.14, "b")
      [], #(_, [a, ..rest] as whole) as it -> "constructor"
      _, v11 -> fn(v12, v13) { v12 }("constructor", 0.5)
    }
    True
  }
  echo {
    {
      {
        10.0
      } *. {
        0.5
      }
    } +. {
      0.5
    }
  } == {
    0.5
  }
}
