pub const tag_value: Bool = True

pub type Promise {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(z: Bool) -> List(Int) {
fn(v0) { case "bc" {
    "res" <> _ -> []
    "data" <> inner -> [5]
    b -> [0]
  } }(False)
}

fn f1(v: String, z: String, v1: List(Int)) -> Int {
42
}

fn f2(s: Bool, length: Promise, v2: Int) -> String {
""
}

pub fn main() {
  let tag_value = case #("bc", 3.14), 0 {
    #(_, v3) as whole, 0 -> "bc"
    #(_, 3.14), 3 -> ""
    #("b", 10.0) as whole, 4 -> "constructor" <> "ab"
    _, _ -> "a"
  }
  let x = {
    3.14
  } +. {
    3.14
  }
  echo case spin(4, 3) {
    item -> {
      item + item
    } + 4
    0 -> {
      fn(v4, v5) { tag_value }(3, False)
    } |> f1({
      let v = True
      let rest = [3, 0]
      "a"
    }, [])
  }
}
