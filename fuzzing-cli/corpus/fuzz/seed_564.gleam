pub const seed_value: Float = 2.0
pub const golden_value: Bool = True

pub type Object {
  Cv0(value: String, inner: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(default: Bool, delete: Int, v1: Int) -> String {
{
    case Cv0("x", True) {
      Cv0("abc", self_) -> "a" <> "x"
      Cv0(b, _) -> "a" <> b
      Cv0("constructor", False) -> "abc"
    }
  } <> {
    fn(v2) { "constructor" }(False)
  }
}

fn f1(v: String, v3: String, v4: Object) -> Bool {
{
    {
      v <> v
    } <> {
      v3 <> "res"
    }
  } == delete(True, {
    let v3 = ""
    2
  }, fn(v5, v6) { v5 }(42, "b"))
}

pub fn main() {
  let golden_value = seed_value
  let golden_value = case {
      let l = 2
      Cv0("constructor", False)
    } {
    Cv0(_, b) if b -> 1.5
    Cv0("abc", seed_value) -> 0.25
    this_ -> {
      let l = []
      0.5
    }
  }
  echo fn(v7, v8) { {
    let golden_value = [5]
    "res"
  } }("abc", 3)
  echo delete({
    "a" <> "res"
  } != {
    {
      let golden_value = ""
      golden_value
    }
  }, 42, 7 + {
    5 + 100
  })
}
