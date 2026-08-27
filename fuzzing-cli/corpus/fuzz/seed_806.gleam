fn f0(m: String) -> Int {
{
    case #("data", [5, 0]) {
      #(_, [5, _, ..]) -> 0 + 0
      #("a", [3, ..rest]) -> 0
      #("x", [_, ..rest]) -> 0
      _ -> fn(v0, v1) { 3 }(1.5, 3)
    }
  } % 6
}

fn f1(v2: String, acc: Bool) -> List(Int) {
case fn(v3) { 3.14 }(2.0) {
    self_ -> [5]
    0.1 -> {
      let v = [100]
      let v = {
        let v2 = v
        let m = acc
        100
      }
      [10, 42]
    }
  }
}

fn default(constructor: Int) -> String {
"x" <> {
    case <<"b":utf8>>, [7] {
      <<"bc":utf8>> as whole, [constructor] if constructor <= 8 -> "" <> "abc"
      _, [constructor, ..rest] -> "ab"
      _, v4 -> "b"
    }
  }
}

pub fn main() {
  let new = case 2.0 {
    0.5 | 0.1 -> f0("x")
    1.0 -> 10
    _ -> 42
  }
  let l = "abc" <> "abc"
  echo [0, 5]
  echo {
    {
      {
        0.0
      } -. {
        0.25
      }
    } *. {
      1.5
    }
  } +. {
    {
      {
        2.0
      } -. {
        0.1
      }
    } +. {
      {
        2.0
      } +. {
        0.0
      }
    }
  }
  echo 10
}
