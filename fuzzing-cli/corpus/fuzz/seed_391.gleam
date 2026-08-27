pub const seed_value: Float = 2.0
pub const euler_value: Int = 0

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(class: Bool) -> String {
case #("ab", []), 100 {
    #(_, [9, 7, ..]), 5 -> "abc"
    #(_, [9]), _ -> case #(1, False), fn(v2, v3) { Cv1 }(0.25, 0.5) {
      #(0, _), Cv1 -> "data" <> "constructor"
      #(7, True), Cv1 -> ""
      _, _ -> "abc"
    }
    #("x", [] as whole) as it, _ -> case {
        let new = 42
        new
      } {
      b -> fn(v4, v5) { v4 }("bc", 7)
      constructor -> "res"
    }
    v6, v7 -> "x"
  }
}

fn f1(v8: Int, v9: Float) -> Float {
v9
}

pub fn main() {
  let default = 5 * 3
  echo {
    {
      let y = 0.5
      2 >= euler_value
    }
  } |> f0()
  echo case "abc" <> "ab", 0.0 {
    euler_value, 0.5 -> {
      {
        let length = default
        seed_value
      }
    } <=. seed_value
    _, _ -> True
  }
  echo seed_value +. {
    2.0
  }
}
