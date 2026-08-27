pub const limit_value: Int = 10
pub const seed_value: String = "b"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: List(Int), delete: Int) -> List(Int) {
v0
}

fn export(m: Bool) -> Int {
{
    let constructor = "x" <> {
      "b" <> "data"
    }
    let m = fn(v1) { fn(v2) { "ab" }(7) }(0.0)
    walk([5], 0)
  }
}

pub fn main() {
  let seed_value = {
    1.5
  } -. {
    {
      1.5
    } +. {
      1.5
    }
  }
  let limit_value = 0.5
  echo f0("x", [], export(True))
  echo fn(v3) { {
    10.0
  } >=. {
    10.0
  } }(7)
  echo case fn(v4, v5) { [] }(100, "b"), "x" {
    [4, 4, ..], "x" -> 5
    [limit_value], _ -> {
      fn(v6) { 3 }(42)
    } % 2
    [], "data" -> {
      {
        let this_ = 2
        7
      }
    } - 42
    _, _ -> {
      {
        let acc = 42
        2
      }
    } + 5
  }
  echo case <<"abc":utf8, "bc":utf8>>, export(False) {
    <<_:utf8, 42:4>>, 9 -> "ab"
    <<"constructor":utf8, 2:16, "ab":utf8>>, 2 -> "a"
    _, 5 -> "x"
    _, v7 -> "bc"
  }
}
