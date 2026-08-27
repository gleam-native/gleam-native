pub const tag_value: Float = 0.0
pub const pi_value: Float = 1.0
pub const golden_value: Int = 5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(Float, List(Int)), z: Float, length: Int) -> Int {
fn(v0) { length * {
    length - v0
  } }(1)
}

pub fn main() {
  let constructor = fn(v1) { {
    let self_ = True
    False
  } }(42)
  echo {
    {
      golden_value + golden_value
    } - {
      golden_value + golden_value
    }
  } * {
    case {
        let x = 100
        let v = pi_value
        v
      }, {
        let self_ = [5]
        let arguments = golden_value
        [4, 10]
      } {
      2.0, [_, ..rest] -> {
        let pi_value = constructor
        2
      }
      _, [2, ..rest] -> walk([5, 0], 1)
      3.14, [] -> walk([5], golden_value)
      v2, v3 -> golden_value + golden_value
    }
  }
}
