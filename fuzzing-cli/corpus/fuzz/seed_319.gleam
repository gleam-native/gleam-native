pub type V0 {
  None(value: String, inner: Int)
  Cv1(String)
}

pub type V2 {
  Cv3
  Cv4(value: Float, inner: String)
  Ok(Int, Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v5: List(Int)) -> String {
fn(v6) { case <<4:16, 42:8>>, v6 *. {
      0.1
    } {
    <<_:utf8>>, 0.25 as whole -> "data" <> "b"
    <<42:16>>, 1.5 -> fn(v7, v8) { "b" }(1.5, 10.0)
    v9, v10 -> ""
  } }(0.1)
}

fn f1(v11: #(Float, Float), class: V0, v12: Int) -> List(Int) {
[10, 7]
}

pub fn main() {
  let constructor = {
    {
      let constructor = 0.0
      0.1
    }
  } -. {
    0.5
  }
  let this_ = False
  echo 2.0
  echo 100 + {
    spin(0, 5) - 100
  }
  echo case [], 10 != 2 {
    [_, ..rest], _ -> fn(v13, v14) { v13 +. v13 }(3.14, False)
    [] as whole, True -> constructor +. {
      0.1
    }
    _, v15 -> {
      let this_ = f1(#(2.0, 2.0), Cv1("bc"), 7)
      2.0
    }
  }
  echo case {
      let s = []
      let default = True
      "ab"
    }, 5 {
    "constructor" <> rest, _ if rest == "abc" -> fn(v16, v17) { f0([7]) }(False, "ab")
    v18, v19 -> v18
  }
}
