pub const golden_value: Float = 1.0
pub const limit_value: String = "data"
pub const tag_value: Int = 5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(y: #(Bool, String)) -> Float {
{
    case {
        let length = True
        3.14
      } {
      3.14 -> {
        0.0
      } *. {
        0.25
      }
      prototype -> fn(v0) { prototype }(False)
      constructor -> constructor -. {
        1.5
      }
    }
  } -. {
    10.0
  }
}

fn class(rest: Int, length: List(Int), s: Int) -> Int {
42
}

pub fn main() {
  let n = case <<0:1>> {
    <<4:16>> -> [4, 10]
    <<_:16, "abc":utf8, "a":utf8>> -> [0, 1]
    _ -> {
      let m = False
      let z = 2.0
      [10, 10]
    }
  }
  let tag_value = {
    tag_value % 6
  } % 3
  echo limit_value
  echo {
    let z = n
    let m = z
    "b"
  }
}
