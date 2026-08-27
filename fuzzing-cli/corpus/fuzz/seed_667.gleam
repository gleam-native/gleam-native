pub type V0 {
  Cv1
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(new: #(String, Int), m: #(String, String), v3: Bool) -> String {
case <<"abc":utf8, "res":utf8, "data":utf8>> {
    <<4:8, _:utf8>> -> "abc"
    <<1:16>> as whole -> "x"
    _ -> {
      "a" <> ""
    } <> "b"
  }
}

fn export(v4: #(List(Int), String), v5: Bool) -> String {
{
    case Cv1 {
      Cv2 | Cv1 -> "b"
      Cv1 -> f0(#("constructor", 4), #("data", "data"), v5)
    }
  } <> f0(fn(v6, v7) { #("constructor", 1) }(True, 42), fn(v8, v9) { #("abc", "a") }("data", 3), v5)
}

pub fn main() {
  let y = 100
  echo {
    case Cv1 {
      Cv1 -> {
        0.5
      } *. {
        1.5
      }
      Cv1 -> {
        0.0
      } *. {
        2.0
      }
      _ -> 0.5
    }
  } +. {
    {
      let y = [1, 2]
      let y = fn(v10, v11) { True }(10.0, 1.0)
      10.0
    }
  }
}
