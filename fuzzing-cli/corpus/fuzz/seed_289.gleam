pub const tag_value: String = "constructor"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, item: Bool, y: Int) -> Float {
{
    0.0
  } *. {
    case "a", <<"ab":utf8, "ab":utf8, "data":utf8>> {
      _, <<2:8>> -> {
        let delete = constructor
        0.0
      }
      "b", <<"data":utf8>> -> 0.25
      "bc", _ -> 0.1
      v0, v1 -> {
        let l = y
        let l = 1.0
        l
      }
    }
  }
}

fn f1(v2: Bool, v3: List(Int)) -> Int {
42
}

pub fn main() {
  let tag_value = case fn(v4) { False }("b") {
    True -> 42
    item -> {
      let prototype = [42]
      1
    }
    True | False -> 2
  }
  let m = False
  echo m
  echo "a" <> {
    fn(v5, v6) { fn(v7) { "abc" }(10.0) }("data", "ab")
  }
  echo [5]
}
