pub const euler_value: Int = 5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(value: Int) -> Bool {
case [42] |> walk(walk([3, 0], 100)) {
    _ -> fn(v0, v1) { True }(10, "ab")
    constructor -> case [10], "abc" {
      [6] as whole, "bc" -> {
        1.0
      } == {
        2.0
      }
      [_], "res" -> False
      [_], "res" <> rest -> False
      _, _ -> True
    }
  }
}

fn export(v2: Bool) -> String {
case {
      0.25
    } +. {
      0.1
    }, fn(v3, v4) { v4 }(100, "abc") {
    v5, _ -> ""
    v6, _ -> {
      "ab" <> "bc"
    } <> "x"
  }
}

pub fn main() {
  let euler_value = True
  let euler_value = {
    let euler_value = {
      let euler_value = [2, 2]
      let m = 42
      euler_value
    }
    0.5
  }
  echo case "" <> "" {
    a | "a" <> a -> a
    v7 -> "abc"
  }
  echo euler_value
  echo 10
  echo 5
}
