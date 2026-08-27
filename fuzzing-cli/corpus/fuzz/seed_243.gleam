pub const pi_value: Float = 2.0

pub type V0 {
  Some(value: String, inner: List(Int))
  Cv1(List(Int))
  Ok
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: V0, v: Float) -> Float {
0.5
}

pub fn main() {
  let pair = {
    fn(v3, v4) { v3 }(0.0, True)
  } +. {
    pi_value +. pi_value
  }
  let acc = [5, 5]
  echo {
    case pi_value *. {
        1.5
      } {
      100.0 | 100.0 -> "abc" <> "res"
      10.0 as whole -> {
        let length = acc
        "a"
      }
      _ | 2.0 -> "data" <> "constructor"
    }
  } <> {
    case <<"bc":utf8, "res":utf8>> {
      <<"constructor":utf8>> -> "a" <> "constructor"
      <<_:little-unsigned-8, s:4>> if s % 2 == 0 -> "data"
      _ -> "bc"
    }
  }
  echo acc
  echo case fn(v5, v6) { acc }(1.0, "bc") {
    [8] -> acc
    [] -> fn(v7) { [7, 42] }(True)
    v8 -> case 0, Cv1([4]) {
      _, Some("" <> rest as whole, [0, ..tail]) -> v8
      2, pair -> []
      4, Ok -> [2, 42]
      v9, _ -> [42]
    }
  }
  echo case 7 {
    6 -> False
    b -> spin(b, b) != 7
    3 | 9 -> case 1 |> spin(7) {
      pair -> "constructor" != "a"
      0 -> {
        3.14
      } <. {
        0.0
      }
      a -> True
    }
  }
}
