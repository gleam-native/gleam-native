pub const seed_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, acc: #(List(Int), Float), length: String) -> List(Int) {
case walk([], constructor) {
    b -> case <<"a":utf8>> {
      <<_:utf8, 100:16, _:utf8>> -> {
        let acc = []
        acc
      }
      _ -> [2, 10]
    }
    8 -> case "res" <> "a" {
      _ -> fn(v0) { [0, 4] }(0.0)
      inner -> fn(v1, v2) { [4, 0] }(2, "constructor")
      _ -> []
    }
    a -> [10, 7]
  }
}

pub fn main() {
  let seed_value = case 4 - 10 {
    2 | 8 -> fn(v3, v4) { v4 }(4, "abc")
    _ -> seed_value <> "a"
    3 | 1 -> seed_value
  }
  echo seed_value <> seed_value
}
