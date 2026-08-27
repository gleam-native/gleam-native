pub type V0 {
  Error(value: String, inner: String)
}

fn f0(m: Int, x: V0) -> Int {
{
    {
      let n = m
      5
    }
  } - {
    case "constructor", m {
      _, 3 -> m - m
      "b" <> _ as whole, 4 if whole != "data" || whole != "ab" -> m
      "a", _ -> {
        let m = 3.14
        0
      }
      _, v1 -> 0
    }
  }
}

fn f1(acc: Bool) -> List(Int) {
[0, 2]
}

fn arguments(v2: V0, rest: Int, class: List(Int)) -> Float {
0.1
}

pub fn main() {
  let x = fn(v3) { [4, 3] }(2)
  echo case Error("res", "data") {
    _ -> {
      let item = [4, 2]
      let l = 0
      f1(True)
    }
    Error(default, "a") -> case [], fn(v4, v5) { x }("constructor", False) {
      [x, 2, ..], [] if x <= 3 && x > 8 -> [1, 42]
      [], [_, ..rest] -> []
      [], [_] -> []
      v6, v7 -> True |> f1()
    }
  }
  echo case "bc" <> "b" {
    "bc" <> inner | "x" <> inner -> inner
    "a" -> "bc"
    "b" <> _ as whole -> case <<"b":utf8>>, 3.14 {
      <<_:utf8, _:utf8, 5:1>>, 100.0 -> "data"
      _, _ -> "bc" <> whole
    }
    v8 -> fn(v9, v10) { v8 <> "b" }(10, "res")
  }
  echo {
    case Error("constructor", "b") {
      Error(_, "data") -> fn(v11, v12) { 0.0 }(True, True)
      _ | Error(_, _) -> arguments(Error("data", "data"), 100, x)
    }
  } +. {
    3.14
  }
}
