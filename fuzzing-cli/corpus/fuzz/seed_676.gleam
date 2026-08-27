pub const seed_value: Bool = False
pub const tag_value: String = "ab"
pub const limit_value: Bool = True

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(value: #(Bool, List(Int))) -> Float {
10.0
}

fn f1(default: String) -> String {
fn(v0) { {
    let v0 = constructor(#(False, []))
    {
      let prototype = True
      default
    }
  } }(2)
}

fn export(v1: List(Int), v2: Float, l: String) -> Float {
{
    {
      3.14
    } +. v2
  } *. constructor(fn(v3) { #(True, [4, 42]) }("bc"))
}

pub fn main() {
  let length = {
    5 + 100
  } - 3
  echo seed_value
  echo 2
  echo {
    let length = case <<"":utf8, 0:8>> {
      <<_:utf8>> -> limit_value
      _ -> False
    }
    case #("ab", "constructor") {
      inner -> [3]
      #("" <> rest, "data") -> []
      #("a", "constructor") -> [3, 10]
    }
  }
  echo case 1 * 1 {
    _ | 7 -> 0.1
    8 as whole -> 0.1
  }
}
