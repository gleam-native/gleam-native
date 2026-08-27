pub const pi_value: Bool = False
pub const tag_value: Bool = True

fn f0(m: String, v: Int, v0: Bool) -> Float {
case 100 - v {
    inner -> 2.0
    3 -> 3.14
    constructor -> {
      {
        0.1
      } +. {
        1.5
      }
    } -. {
      {
        0.1
      } *. {
        0.25
      }
    }
  }
}

fn f1(default: #(Float, Float), pair: #(Float, List(Int)), v1: Bool) -> String {
case 0, {
      1.5
    } == {
      3.14
    } {
    4, _ -> {
      "a" <> "a"
    } <> {
      "ab" <> "abc"
    }
    7 as whole, False -> {
      "x" <> "data"
    } <> "a"
    _, _ -> "a"
  }
}

fn class(v2: #(Bool, String), y: Int, v3: Int) -> Bool {
False
}

pub fn main() {
  let length = case fn(v4) { #([0, 10], "bc") }("ab"), "bc" <> "b" {
    #([b, ..rest], "x"), "a" if b % 2 == 0 -> rest
    #([5, 2, ..], "b") as whole, tag_value if tag_value != "b" || tag_value != "b" -> [1, 5]
    #([_] as whole, "bc" <> rest as it), "data" -> [3]
    _, v5 -> []
  }
  let constructor = case "abc" <> "a" {
    "b" <> a -> f1(#(0.25, 3.14), #(3.14, []), pi_value)
    _ -> "a" <> "abc"
    inner -> inner
  }
  echo case 7, [] {
    7, [7] as whole -> {
      fn(v6) { 3 }(True)
    } < {
      {
        let pi_value = "abc"
        0
      }
    }
    prototype, [tag_value] -> "a" == constructor
    _, [] -> class(#(False, "data"), 4, {
      let class = "constructor"
      let constructor = 0.0
      3
    })
    _, _ -> 5 != 4
  }
  echo constructor
  echo class(case fn(v7) { constructor }(True) {
    "res" as whole if whole == "bc" || whole != "data" -> fn(v8, v9) { #(False, "x") }(True, 0.25)
    _ -> #(False, "a")
  }, case "b" |> f0(fn(v10) { 2 }("data"), fn(v11) { True }("res")), fn(v12) { length }("abc") {
    100.0, [] -> 2 - 2
    1.5, [3, ..rest] -> 7
    0.0, [2] -> 1
    _, _ -> 3
  }, 100)
}
