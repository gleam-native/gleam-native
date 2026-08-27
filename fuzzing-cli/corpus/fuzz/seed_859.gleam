pub type V0 {
  Error(value: String, inner: String)
}

fn f0(prototype: V0) -> Bool {
False
}

fn f1(v1: Float, m: Bool) -> Int {
0
}

pub fn main() {
  let default = fn(v2, v3) { True }("res", 2.0)
  echo default
  echo {
    {
      fn(v4) { "ab" }(10)
    } <> {
      "res" <> "b"
    }
  } <> "b"
  echo case Error("bc", "abc") {
    constructor -> 0.1
    Error("" <> rest, _) -> {
      10.0
    } *. {
      fn(v5) { 100.0 }(7)
    }
  }
  echo {
    "abc" <> {
      "" <> "data"
    }
  } == {
    case 10 - 42 {
      _ -> "abc" <> "b"
      v6 -> "x"
    }
  }
}
