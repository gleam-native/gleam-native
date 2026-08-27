pub const tag_value: Int = 2
pub const limit_value: Float = 2.0
pub const euler_value: String = "abc"

pub type V0 {
  Number(value: String, inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(value: #(Int, Float), l: List(Int), acc: V0) -> List(Int) {
{
    let l = case {
        let new = 3
        acc
      } {
      Number("b", item) -> item +. {
        10.0
      }
      Number("abc", 1.0) | Number(_, _) -> 2.0
      v1 -> {
        let new = l
        0.1
      }
    }
    case [7, 2], 2 |> spin(100) {
      [0], 1 -> [4]
      [h, l, ..], 5 -> {
        let n = "abc"
        let s = [0]
        s
      }
      [constructor, 5, ..], arguments -> []
      _, v2 -> [1, 0]
    }
  }
}

fn f1(constructor: Int, default: Bool, v3: Bool) -> Float {
1.5
}

pub fn main() {
  let class = 4
  echo tag_value
}
