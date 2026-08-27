pub const tag_value: Bool = False
pub const euler_value: Bool = False

pub type Map {
  Cv0(value: String, inner: String)
}

pub type V1 {
  Ok(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn new(item: V1, s: String, prototype: #(Float, Float)) -> List(Int) {
case {
      let v = 0.1
      let s = [10]
      4
    }, {
      0.1
    } *. {
      0.5
    } {
    1, 3.14 -> case "abc" {
      v2 -> [42]
      "res" -> []
    }
    8, v3 -> case item, #("constructor", True) {
      Ok("" <> rest), #("constructor" as whole, _) -> {
        let arguments = [5]
        arguments
      }
      v4, #(_, _) -> [3, 42]
      _, v5 -> [7, 7]
    }
    _, _ -> [0]
  }
}

pub fn main() {
  echo spin(7, spin(42 + 3, 3 - 1))
}
