pub const tag_value: Int = 100
pub const golden_value: Bool = False
pub const pi_value: Int = 3

pub type V0 {
  Cv1(value: List(Int))
  Cv2(String, value: List(Int))
  Cv3(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(length: Bool, v4: Int, v5: V0) -> Float {
{
    {
      {
        0.0
      } +. {
        100.0
      }
    } +. {
      2.0
    }
  } *. {
    0.25
  }
}

pub fn main() {
  let delete = case <<"a":utf8>>, tag_value - 0 {
    <<"abc":utf8>>, y -> pi_value >= y
    _, 7 -> False
    _, _ -> False
  }
  let default = 5 * 2
  echo case golden_value {
    item -> f0(golden_value, 100, Cv1([42, 5])) +. {
      100.0
    }
    False -> f0(True, 10, Cv2("", [])) *. {
      0.25
    }
    False | True -> 10.0
  }
  echo fn(v6, v7) { case "data" != "x", Cv3("b") {
    True, Cv1([a]) if a > 2 || a % 2 == 0 -> {
      let constructor = False
      let length = [100, 100]
      delete
    }
    _, Cv2("abc", [] as whole) -> {
      0.5
    } <=. v7
    _, v8 -> golden_value
  } }(2, 1.0)
  echo case "bc" <> "b" {
    "abc" | "constructor" <> _ -> [3]
    "x" <> inner | "ab" <> inner -> case !delete, "bc" {
      True, "b" -> [5, 10]
      pair, v9 -> {
        let z = tag_value
        let new = default
        [2]
      }
      new, "bc" -> []
    }
    _ -> [100, 2]
  }
  echo case {
      let this_ = pi_value
      let default = []
      Cv3("x")
    }, [7, 10] {
    Cv2("x", [b, ..rest]) as whole, [_] as it -> delete
    Cv1([9, ..rest]) as whole, [h, x, ..] -> case True {
      v10 -> False
      True -> golden_value
    }
    v11, _ -> golden_value
  }
}
