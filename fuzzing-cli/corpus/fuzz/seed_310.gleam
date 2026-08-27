pub type Object {
  Cv0(value: String, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(y: Int) -> Bool {
fn(v1, v2) { False }(False, 0.5)
}

fn f1(acc: List(Int), item: Int, v3: Int) -> String {
"constructor" <> "x"
}

fn constructor(x: String, v4: Int) -> Bool {
fn(v5) { True }("x")
}

pub fn main() {
  echo {
    !{
      "res" != "res"
    }
  } || {
    fn(v6, v7) { True }("a", True)
  }
  echo []
  echo case <<"data":utf8, 100:16, 0:16>> {
    <<this_:16, _:utf8>> -> case fn(v8, v9) { [1] }(0.25, 42) {
      [] -> {
        let constructor = True
        let this_ = this_
        this_
      }
      [a] if a > 7 -> a
      [] -> [] |> walk(7)
      _ -> fn(v10) { this_ }("x")
    }
    <<_:utf8>> -> case [2, 4] {
      [8, 7, ..] -> 7
      [x, constructor, ..] -> fn(v11) { x }("")
      _ -> 4
    }
    <<"a":utf8>> -> 0
    v12 -> 1
  }
  echo {
    "a" <> {
      "a" <> "x"
    }
  } <> f1([42, 5], {
    let this_ = "ab"
    let this_ = True
    10
  }, walk([], 2))
}
