pub const pi_value: String = "constructor"
pub const tag_value: Float = 1.5
pub const golden_value: Float = 1.5

pub type V0 {
  None(value: String, inner: List(Int))
  Cv1(value: String)
  Some(value: Float, inner: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(v2: V0) -> String {
fn(v3) { "" <> {
    {
      let rest = 7
      let v2 = rest
      v3
    }
  } }("")
}

fn f1(v4: V0, constructor: String, prototype: Float) -> Float {
prototype
}

fn f2(v5: Int, arguments: Float) -> Bool {
case <<"b":utf8>>, v5 {
    <<_:4>>, _ -> True
    <<42:8, constructor:8, _:bytes>>, _ -> {
      let arguments = {
        let y = False
        let pair = []
        [42, 4]
      }
      True
    }
    _, 3 as whole -> True
    v6, v7 -> {
      "abc" <> "b"
    } != {
      {
        let v6 = False
        "abc"
      }
    }
  }
}

pub fn main() {
  let pair = pi_value <> ""
  let self_ = f1(None("", []), pair, golden_value) -. {
    0.5
  }
  echo case {
      let self_ = [2]
      2
    } {
    item -> case {
        let item = 4
        item
      } {
      constructor -> []
      _ -> {
        let v = item
        []
      }
    }
    1 -> case 7 {
      constructor -> [3]
      b -> []
    }
  }
  echo 3 |> f2(fn(v8, v9) { v9 }(False, 1.0))
  echo pi_value
}
