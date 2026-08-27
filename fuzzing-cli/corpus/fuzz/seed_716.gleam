pub const limit_value: String = "b"

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(delete: Bool) -> String {
"constructor" <> {
    case 3 {
      v0 -> "abc" <> "data"
      0 -> {
        let length = 3.14
        let prototype = [1]
        "abc"
      }
      _ -> "b"
    }
  }
}

fn f1(new: #(Bool, Float), arguments: #(Float, String), acc: Int) -> String {
{
    let class = 2 - 3
    "b"
  }
}

fn f2(v1: Float, pair: Float) -> Bool {
True
}

pub fn main() {
  let limit_value = case #([5, 100], 0.25) {
    inner -> {
      3.14
    } -. {
      0.0
    }
    #([8] as whole, 0.5) -> 3.14
    a -> 3.14
  }
  let limit_value = constructor(False || False)
  echo 4 * {
    {
      let m = 10 + 100
      let z = fn(v2, v3) { v3 }("constructor", 3)
      fn(v4) { 100 }(10)
    }
  }
}
