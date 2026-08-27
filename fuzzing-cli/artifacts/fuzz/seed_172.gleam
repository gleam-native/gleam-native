fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, value: Float, v0: Int) -> List(Int) {
[]
}

fn f1(z: Bool, v1: List(Int), constructor: Float) -> Int {
3
}

fn f2(m: #(Int, Float), pair: Int) -> String {
"a"
}

pub fn main() {
  let arguments = case "a" {
    "bc" <> rest -> 0.1
    "x" <> item | "" <> item -> {
      2.0
    } /. {
      0.5
    }
    _ -> 0.0
  }
  let acc = f2({
    let arguments = 4
    let default = "constructor"
    #(4, 0.1)
  }, 7)
  echo case "data", arguments <=. arguments {
    "abc" <> rest, False -> case 100 {
      0 | 3 -> False
      9 as whole -> True
      _ -> False
    }
    "abc" as whole, _ -> case 42 + 7 {
      4 | 7 -> fn(v2, v3) { True }(1, 2.0)
      5 | 9 -> True
      6 | 8 -> True
      _ -> 100 == 3
    }
    v4, v5 -> case v4, v5 {
      "a", v6 if !v6 -> {
        0.5
      } == arguments
      arguments, _ -> 5 >= 2
    }
  }
}
