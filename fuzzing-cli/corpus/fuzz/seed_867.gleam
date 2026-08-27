pub const limit_value: Float = 2.0
pub const seed_value: Bool = True
pub const tag_value: String = "bc"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(self_: String) -> Bool {
{
    let length = case 5 {
      self_ -> [1]
      5 -> []
    }
    let acc = 10
    case 5 {
      a -> acc == a
      b -> False
    }
  }
}

fn f1(m: #(String, Float), y: Int) -> String {
case <<"b":utf8, "constructor":utf8>> {
    <<7:8, "abc":utf8>> -> "ab"
    <<0:16>> as whole -> "data"
    _ -> "constructor" <> {
      "data" <> "abc"
    }
  }
}

pub fn main() {
  echo {
    {
      {
        2.0
      } +. limit_value
    } *. {
      limit_value +. {
        2.0
      }
    }
  } /. {
    3.14
  }
}
