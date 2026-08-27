pub const tag_value: String = "a"
pub const golden_value: String = "x"
pub const euler_value: String = "constructor"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: String, item: #(Int, Float), v3: Bool) -> String {
v2
}

fn f1(m: Int, acc: Bool) -> List(Int) {
[42, 10]
}

fn yield(y: V0) -> Bool {
False
}

pub fn main() {
  echo 100
  echo case golden_value <> "ab", <<4:16, "data":utf8>> {
    "res", <<_:8>> -> case f0("ab", #(10, 0.1), False) {
      "bc" -> 2
      "ab" <> a if a != "constructor" -> {
        let default = 100.0
        let arguments = True
        4
      }
      "b" | "x" -> 7 - 5
      v4 -> 0 % 4
    }
    "constructor", <<3:8>> -> spin(2, 4 |> spin({
      let y = 4
      let m = []
      y
    }))
    "a", _ -> fn(v5, v6) { v6 * v6 }(True, 3)
    _, _ -> 42
  }
  echo {
    euler_value <> {
      golden_value <> "data"
    }
  } <> {
    case {
        let tag_value = 0
        let x = 0.0
        42
      } {
      6 -> f0(golden_value, #(4, 0.25), False)
      3 | 5 -> "constructor"
      v7 -> "res"
    }
  }
  echo {
    {
      1 + 0
    } + {
      fn(v8, v9) { 4 }("res", 0.1)
    }
  } - {
    {
      fn(v10, v11) { 42 }(1, 42)
    } - {
      {
        let tag_value = golden_value
        3
      }
    }
  }
}
