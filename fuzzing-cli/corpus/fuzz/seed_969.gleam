pub const seed_value: Bool = True
pub const euler_value: Int = 1
pub const golden_value: String = "a"

pub type V0 {
  None(value: String, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(rest: V0, n: Int, v1: List(Int)) -> Float {
{
    0.1
  } -. {
    case {
        2.0
      } -. {
        0.1
      }, #(2, "bc") {
      _, #(3, "b") -> 0.0
      new, #(v2, "x" as whole) -> 0.25
      v3, _ -> v3
    }
  }
}

fn f1(arguments: Int) -> List(Int) {
[10]
}

fn extends(v4: String, n: Int, v5: Int) -> Bool {
False || {
    case "data" {
      _ | "data" -> False
      constructor | "data" <> constructor -> True
      constructor -> False
    }
  }
}

pub fn main() {
  echo 100
  echo golden_value
  echo "res"
  echo {
    {
      fn(v6) { 0 }(3.14)
    } - {
      {
        let seed_value = golden_value
        let s = euler_value
        5
      }
    }
  } - {
    42 |> spin(100)
  }
}
