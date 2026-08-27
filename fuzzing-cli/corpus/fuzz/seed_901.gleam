fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(Bool, String), value: List(Int), l: String) -> List(Int) {
fn(v0, v1) { case 0.0 {
    _ -> {
      let item = 4
      let x = item
      value
    }
    10.0 -> {
      let v1 = 10
      let s = l
      value
    }
    b -> {
      let v1 = b
      value
    }
  } }(True, False)
}

fn new(v2: Int) -> Int {
v2
}

pub fn main() {
  echo {
    "x" <> "x"
  } <> "data"
  echo case {
      let v = False
      ""
    } {
    "b" <> rest | "b" <> rest -> fn(v3) { 0 }(0.1)
    _ -> 3
    y | "" <> y -> {
      [] |> walk(fn(v4, v5) { 0 }(False, 100))
    } + 100
  }
  echo {
    let default = False
    let length = case {
        let rest = [100]
        let v = 5
        [4, 1]
      } {
      [constructor] -> "x" == "a"
      [] -> {
        let z = 2
        let default = "bc"
        True
      }
      v6 -> True
    }
    "abc"
  }
  echo case "x" <> "b" {
    "abc" <> rest -> case fn(v7) { #("x", 100.0) }(True) {
      #("" <> rest, _) if rest == "ab" && rest == "bc" -> rest
      constructor -> rest <> "x"
      #("abc" <> rest, v8) -> {
        let arguments = v8
        let self_ = True
        rest
      }
    }
    "data" <> rest -> case <<1:16, "ab":utf8, "x":utf8>> {
      <<_:utf8>> -> "x"
      _ -> rest <> "bc"
    }
    "x" <> item -> "bc"
    v9 -> "b"
  }
}
