pub const golden_value: String = "bc"
pub const limit_value: Int = 4

fn yield(constructor: #(Bool, List(Int)), v0: Int, length: #(Float, Bool)) -> Float {
3.14
}

fn f1(acc: Float, v1: Int) -> List(Int) {
[100]
}

pub fn main() {
  echo 5
  echo {
    case "b" <> golden_value, 1 {
      _, _ -> 42
      "bc", golden_value -> 5 - 5
    }
  } + {
    case "ab" {
      _ -> limit_value
      "res" | "res" <> _ -> {
        let length = 2.0
        let limit_value = limit_value
        1
      }
      "a" -> 7
    }
  }
  echo case limit_value + limit_value {
    inner -> {
      fn(v2) { 0 }(3.14)
    } * 7
    _ | 0 -> 3
  }
  echo #(False, []) |> yield({
    let prototype = 0.5
    let length = True
    limit_value
  }, #(1.0, True))
}
