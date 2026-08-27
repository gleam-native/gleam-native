pub const limit_value: Int = 4

pub type Number {
  Cv0(value: String, inner: Int)
  Cv1(value: Int, inner: Bool)
}

pub type V2 {
  Cv3(value: String, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(n: Int) -> Float {
1.5
}

fn arguments(n: Bool, v4: V2, v5: Int) -> String {
fn(v6, v7) { {
    fn(v8, v9) { v6 }(5, "bc")
  } <> {
    {
      let arguments = []
      v6
    }
  } }("a", 0.1)
}

fn f2(v: #(String, Float), rest: Bool, arguments: V2) -> Int {
{
    case rest || True {
      inner -> 1
      False -> 5
    }
  } + 4
}

pub fn main() {
  let value = arguments(True, {
    let new = limit_value
    let l = 100.0
    Cv3("data", 10)
  }, 3)
  echo case [0, 7] {
    [b] if b > 1 -> {
      let value = "x"
      [42, 100]
    }
    [_, _, ..] -> case f2(#("ab", 0.1), True, Cv3("x", 42)) {
      inner -> {
        let l = value
        [1]
      }
      6 -> fn(v10) { [0, 1] }(3.14)
      1 -> [1]
    }
    [constructor, _, ..] -> []
    _ -> case Cv3("a", 10) {
      Cv3(v11, _) -> fn(v12, v13) { [10, 3] }(0, "ab")
      Cv3("a", limit_value) -> [2, 0]
    }
  }
  echo False
  echo limit_value
}
