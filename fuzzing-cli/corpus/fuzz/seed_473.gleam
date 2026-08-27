pub const limit_value: Float = 100.0
pub const golden_value: Float = 1.5
pub const seed_value: Float = 0.5

pub type V0 {
  Ok(value: String, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(s: Float, v1: Int, v: Int) -> Bool {
case "abc" <> "x", fn(v2, v3) { "abc" }(False, "ab") {
    _, "constructor" -> True
    _, "bc" -> False
    _, _ -> 4 == 0
  }
}

fn static(self_: #(List(Int), List(Int))) -> Bool {
case <<2:8, "b":utf8>> {
    <<_:utf8, "ab":utf8>> -> case <<"x":utf8>>, True {
      <<_:utf8>>, True -> False
      _, False -> delete(100.0, 1, 7)
      v4, _ -> True
    }
    _ -> 2 < 7
  }
}

fn f2(item: Bool, v5: Bool, v: #(Bool, List(Int))) -> String {
case fn(v6) { 100 }(100.0), Ok("abc", 5) {
    _, Ok("bc" <> rest as whole, v) as it if rest == "abc" && whole != "a" -> "b" <> {
      rest <> ""
    }
    0, _ -> {
      fn(v7) { "bc" }(0)
    } <> {
      "bc" <> "b"
    }
    _, v8 -> "abc"
  }
}

pub fn main() {
  let limit_value = case 4, 7 + 0 {
    _, 2 -> []
    _, 6 -> [42, 100]
    _, _ -> fn(v9) { [] }(7)
  }
  let m = fn(v10) { fn(v11, v12) { 3 }(100.0, "x") }(3.14)
  echo fn(v13) { {
    m - 10
  } + {
    {
      let x = [4, 10]
      m
    }
  } }(False)
}
