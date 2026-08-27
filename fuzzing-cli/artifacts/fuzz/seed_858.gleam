fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(arguments: List(Int)) -> Int {
{
    case spin(0, 10) {
      4 -> 42
      item -> item |> spin(4)
    }
  } + 10
}

fn f1(delete: #(Int, List(Int))) -> Bool {
case 100 {
    a -> case True && True {
      constructor -> constructor
      inner -> inner
    }
    v0 -> case {
        let default = 1.0
        let delete = "a"
        "res"
      } {
      inner -> fn(v1) { True }(10)
      v2 -> {
        let acc = [5]
        False
      }
      "constructor" <> b -> fn(v3) { True }(10)
    }
    v4 -> case "constructor", #(True, "ab") {
      "ab", #(_, _) -> {
        let v4 = v4
        let x = v4
        True
      }
      "bc" <> rest, #(_, _) -> True
      v5, _ -> True
    }
  }
}

fn f2(v6: Bool) -> Float {
fn(v7) { case 5 - 5 {
    4 -> 100.0
    v6 -> {
      3.14
    } *. {
      0.25
    }
  } }(False)
}

pub fn main() {
  let this_ = "a" <> {
    "x" <> "abc"
  }
  let z = {
    10 |> spin(3)
  } >= {
    {
      let this_ = True
      let prototype = 10
      prototype
    }
  }
  echo f2(!{
    True && z
  })
  echo this_
  echo z
}
