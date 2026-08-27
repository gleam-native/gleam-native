pub const tag_value: Float = 0.5

pub type Symbol {
  Cv0(value: String, inner: Int)
  Cv1(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(v2: String, v3: #(Bool, Int)) -> Int {
case {
      let new = False
      v2
    } {
    item | "abc" <> item -> 42 * 10
    _ -> 3
    "x" <> rest -> {
      fn(v4, v5) { v4 }(7, "constructor")
    } - 5
  }
}

fn f1(pair: #(Int, String), v6: Float, v7: Int) -> String {
case "data" {
    "ab" <> b -> "ab"
    "res" <> constructor -> constructor
    b -> "" <> "bc"
  }
}

pub fn main() {
  let x = case Cv0("", 10) {
    Cv1("res") -> []
    Cv0(constructor, _) -> []
    Cv0(_, item) -> []
    _ -> [1]
  }
  echo tag_value
}
