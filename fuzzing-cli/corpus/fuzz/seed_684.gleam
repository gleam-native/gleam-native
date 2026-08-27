pub const limit_value: Bool = True
pub const euler_value: Int = 5

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(length: Int) -> Bool {
case "b" {
    _ | "ab" -> case #("x", "data") {
      #("x" <> _, "abc" <> _) -> fn(v0) { True }(0.0)
      #(_, "x") | #("ab" <> _, "data") -> fn(v1, v2) { True }(100, 1)
      #("a" as whole, "res" as it) -> fn(v3, v4) { False }("data", False)
      _ -> True
    }
    "res" <> _ -> case [0] {
      [6, ..rest] -> {
        let rest = 0.0
        True
      }
      [] -> length <= 10
      _ -> {
        let y = 1.0
        let l = "constructor"
        False
      }
    }
    b -> {
      0.1
    } >=. {
      {
        0.0
      } *. {
        0.0
      }
    }
  }
}

fn f1(constructor: Int) -> Float {
{
    {
      {
        2.0
      } /. {
        10.0
      }
    } *. {
      fn(v5, v6) { 0.5 }(5, False)
    }
  } /. {
    0.5
  }
}

pub fn main() {
  let n = {
    let value = f1(1)
    {
      2.0
    } -. {
      0.1
    }
  }
  echo {
    case <<"res":utf8, "data":utf8>> {
      <<1:16, _:bytes>> as whole -> "abc"
      <<_:big-unsigned-16, _:little-unsigned-8>> as whole -> "ab"
      _ -> "res"
    }
  } <> "abc"
  echo 2
}
