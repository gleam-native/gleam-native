pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: String, pair: Int) -> Bool {
case [] {
    [1, h, ..] as whole if h > 9 && h > 8 -> {
      let v = v2 <> v2
      let whole = fn(v3, v4) { False }("ab", "abc")
      whole || True
    }
    [_] -> False
    _ -> {
      0.25
    } != {
      fn(v5) { 0.1 }(100)
    }
  }
}

fn f1(s: Bool) -> String {
case Cv1([42, 7], 2), 5 - 10 {
    Cv1([], 3), 8 -> case <<"constructor":utf8, "b":utf8>>, [2, 3] {
      <<_:utf8, _:utf8, _:utf8>> as whole, [1, 4, ..] -> {
        let s = True
        "constructor"
      }
      <<0:8>>, [x] -> fn(v6, v7) { "b" }(True, True)
      _, [constructor, _, ..] -> fn(v8, v9) { "abc" }(10.0, 0)
      _, _ -> "data"
    }
    Cv1([7, 8, ..], 5 as whole), _ -> case {
        let delete = s
        "x"
      }, {
        let class = [100, 100]
        let rest = class
        rest
      } {
      "b" <> _, [] as whole -> "a"
      "constructor" <> _, [5, ..rest] -> fn(v10, v11) { "x" }(1.0, True)
      "a" <> rest, [_, ..tail] -> rest
      v12, v13 -> "data"
    }
    _, _ -> {
      let acc = "bc" <> "b"
      let prototype = acc
      "ab"
    }
  }
}

pub fn main() {
  let self_ = 0.1
  echo spin(7 + 0, 1)
  echo False
  echo self_
}
