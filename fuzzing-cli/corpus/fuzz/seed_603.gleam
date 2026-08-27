pub type Promise {
  Cv0(value: String, inner: String)
  None
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(default: Promise, v1: String) -> String {
"a"
}

pub fn main() {
  let pair = {
    fn(v2, v3) { Cv0("abc", "ab") }(5, 1.5)
  } |> yield("abc")
  echo yield({
    let this_ = 2
    None
  }, "abc") == {
    case <<"a":utf8>> {
      <<"a":utf8, "constructor":utf8>> -> yield(Cv0("abc", "data"), "a")
      _ -> fn(v4, v5) { v5 }(False, "bc")
    }
  }
  echo case 100 - 3 {
    a -> {
      a % 5
    } > 3
    2 -> True
    constructor -> False
  }
  echo case [] {
    [] -> 0.1
    [] as whole -> fn(v6) { {
      0.25
    } /. {
      10.0
    } }("data")
    v7 -> {
      {
        let this_ = 0.5
        this_
      }
    } +. {
      {
        2.0
      } +. {
        3.14
      }
    }
  }
}
