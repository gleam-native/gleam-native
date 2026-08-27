pub const seed_value: String = "x"
pub const euler_value: String = "res"

pub type Symbol {
  Cv0(value: String, inner: List(Int))
  Record(value: Float, inner: String)
  Some(value: Float, inner: Int)
}

fn extends(constructor: Float, v1: Symbol, length: String) -> Float {
{
    let constructor = case <<"bc":utf8, 10:16>> {
      <<_:utf8>> -> 42 - 0
      <<2:1>> as whole -> 4
      _ -> 1
    }
    let length = True
    case "" <> "bc" {
      "b" <> rest -> 0.25
      "x" | "bc" -> 0.1
      self_ -> {
        let n = [3]
        let s = "constructor"
        0.25
      }
    }
  }
}

fn new(delete: #(List(Int), List(Int))) -> Float {
1.5
}

fn default(l: Int) -> String {
case Record(0.1, "b") {
    _ -> "abc"
    Cv0(constructor, _) -> {
      "data" <> constructor
    } <> "b"
  }
}

pub fn main() {
  let m = {
    "b" <> "x"
  } <> {
    seed_value <> euler_value
  }
  let acc = fn(v2, v3) { extends(0.1, Record(0.5, "x"), "x") }(False, True)
  echo []
  echo fn(v4, v5) { False }("ab", 3.14)
}
