pub const limit_value: Bool = True
pub const tag_value: Int = 3

pub type Promise {
  Cv0(value: String, inner: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(v1: Int) -> Float {
{
    1.5
  } *. {
    {
      {
        let v1 = [4]
        100.0
      }
    } -. {
      {
        let acc = True
        1.5
      }
    }
  }
}

fn f1(v2: Int, s: String) -> String {
{
    {
      let v2 = v2
      let value = v2
      "ab"
    }
  } <> {
    case s <> s {
      "abc" <> _ | "x" -> s
      "bc" <> _ | "a" -> s <> "res"
      b -> b <> s
    }
  }
}

fn f2(l: Bool, v3: List(Int), v4: String) -> Int {
3
}

pub fn main() {
  let limit_value = {
    fn(v5, v6) { "x" }(0.0, 0)
  } <> "res"
  echo spin(f2(True, [100, 1], limit_value), fn(v7, v8) { 1 }(5, False)) * tag_value
  echo 0.0
  echo {
    fn(v9) { {
      0.0
    } +. {
      0.25
    } }(False)
  } +. {
    fn(v10) { 3.14 }(False)
  }
}
