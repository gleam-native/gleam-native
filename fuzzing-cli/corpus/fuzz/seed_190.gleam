pub type Symbol {
  Cv0(value: String, inner: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(new: Int, length: List(Int)) -> List(Int) {
case "constructor" <> "constructor" {
    "data" <> rest | "res" <> rest -> [2]
    item -> case 0 {
      _ -> length
      5 -> fn(v1) { [42, 5] }(True)
      _ | 2 -> [100, 2]
    }
    "bc" -> case False {
      item -> fn(v2, v3) { length }(True, 0.5)
      _ | False -> []
      _ -> length
    }
  }
}

fn f1(v4: List(Int), item: String, prototype: Int) -> Int {
0
}

fn f2(new: Symbol, class: Bool, z: Float) -> Bool {
True
}

pub fn main() {
  let delete = case {
      let y = []
      let pair = 1.5
      []
    } {
    [] -> fn(v5, v6) { [2, 3] }(0.5, 0.5)
    [9, ..rest] -> []
    [] -> fn(v7, v8) { [] }(False, "abc")
    _ -> f0(7, [2])
  }
  let s = case <<"b":utf8>>, 42 {
    <<_:utf8>>, 6 -> fn(v9) { [100, 5] }(False)
    <<_:utf8, default:16>>, 3 if default == 3 -> delete
    <<_:utf8, acc:4>>, 8 -> delete
    _, v10 -> f0(v10, delete)
  }
  echo f2(Cv0("b", "x"), {
    let s = 10
    let s = True
    s
  }, {
    1.0
  } +. {
    0.5
  }) && {
    f2(Cv0("ab", "ab"), True, 10.0) || True
  }
  echo case "x" <> "res", 2 {
    "bc" <> _ as whole, 8 if whole != "ab" -> {
      let length = 7 - 1
      "abc"
    }
    "x", 7 -> "res"
    _, _ -> "b" <> "x"
  }
}
