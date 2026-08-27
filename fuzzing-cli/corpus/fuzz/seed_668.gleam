pub type V0 {
  Some(value: String, inner: Bool)
  Record
  Cv1(String, Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(new: Int, v2: Float) -> String {
"res"
}

fn class(acc: String) -> Int {
1
}

fn yield(m: Bool, acc: V0, v3: V0) -> Float {
3.14
}

pub fn main() {
  echo 1 |> spin(class("bc"))
  echo {
    let l = case 2.0, <<4:8>> {
      _, <<_:big-signed-8, _:bytes>> -> {
        0.5
      } *. {
        10.0
      }
      100.0, _ -> {
        100.0
      } +. {
        1.0
      }
      v4, _ -> yield(False, Record, Record)
    }
    case fn(v5) { v5 }(3), {
        let y = 2
        10
      } {
      4 as whole, 6 if whole <= 9 && whole == 5 -> 4
      l, 9 as whole if l <= 4 -> 42
      v6, _ -> 3
    }
  }
}
