pub const tag_value: Float = 1.5
pub const golden_value: Int = 100

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: List(Int), pair: Bool, v0: Bool) -> Bool {
case {
      let class = constructor
      let y = 1.5
      "bc"
    }, fn(v1, v2) { pair }(True, 100) {
    _, _ -> {
      True && pair
    } || {
      "constructor" == "constructor"
    }
    pair, self_ -> self_
    "" <> _ as whole, True -> {
      fn(v3, v4) { False }(True, "ab")
    } && {
      v0 && pair
    }
  }
}

fn f1(v5: List(Int), v6: Bool) -> Float {
case "b" {
    "x" <> rest if rest == "bc" -> 0.5
    "abc" <> rest | "a" <> rest -> {
      0.1
    } -. {
      {
        1.0
      } +. {
        100.0
      }
    }
    v7 -> 1.5
  }
}

pub fn main() {
  echo f1([], case "ab" <> "abc" {
    v8 | "x" <> v8 -> False
    constructor -> False
  })
  echo 1.0
  echo 3.14
  echo f0([], [] |> f0(False, True), [] |> f0(7 > 0, {
    let x = golden_value
    let x = golden_value
    True
  }))
}
