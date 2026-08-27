fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: List(Int), new: Bool, y: Int) -> List(Int) {
case #(2, 10.0) {
    a -> [42, 7]
    #(_, class) -> [7]
  }
}

fn delete(v0: Bool, y: Int) -> Int {
case {
      let new = 0.1
      let new = []
      "a"
    } {
    _ | "abc" -> spin(10 * y, 3)
    "abc" <> rest | "ab" <> rest -> y
  }
}

fn f2(v1: Int, y: #(List(Int), String), class: #(Bool, String)) -> Int {
case 100, <<"x":utf8, 0:4>> {
    arguments, <<"bc":utf8>> as whole -> 0
    _, <<100:8>> -> v1
    _, _ -> 5
  }
}

pub fn main() {
  let v = fn(v2, v3) { f2(7, #([100], "ab"), #(False, "ab")) }(True, False)
  let v = 0.25
  echo f0([], {
    fn(v4, v5) { v5 }("ab", False)
  } && False, f2(100 |> spin(7), fn(v6) { #([2, 4], "constructor") }(True), #(False, "x")))
  echo "bc"
  echo case <<"data":utf8, 42:1>> {
    <<"":utf8>> -> [5]
    _ -> []
  }
  echo "abc"
}
