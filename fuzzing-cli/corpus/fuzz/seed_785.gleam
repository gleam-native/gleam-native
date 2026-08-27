pub const pi_value: Float = 0.5

pub type Object {
  Cv0(value: String, inner: String)
  Cv1(value: List(Int))
  Number(Int, value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(z: Bool, v2: Int, y: Float) -> List(Int) {
[3, 4]
}

pub fn main() {
  echo {
    5 * 42
  } - 3
  echo pi_value
  echo {
    let pair = case "constructor" <> "bc" {
      "ab" <> constructor | "ab" <> constructor -> {
        let class = [10, 10]
        10.0
      }
      "bc" <> rest | "b" <> rest -> {
        2.0
      } -. pi_value
      "a" -> pi_value *. pi_value
      _ -> {
        10.0
      } /. {
        0.5
      }
    }
    let self_ = {
      0.1
    } -. pair
    case walk([10, 7], 1) {
      _ -> {
        2.0
      } +. pair
      class -> {
        1.5
      } +. pi_value
    }
  }
}
