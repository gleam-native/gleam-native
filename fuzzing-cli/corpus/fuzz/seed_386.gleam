pub const limit_value: Float = 2.0
pub const euler_value: String = ""
pub const pi_value: String = "bc"

pub type V0 {
  Some(value: String, inner: String)
  Number(value: List(Int), inner: List(Int))
}

pub type Map {
  Cv1(value: List(Int), inner: Bool)
}

fn f0(prototype: String) -> String {
"data"
}

fn f1(arguments: String, delete: #(String, Int)) -> List(Int) {
case {
      let value = []
      False
    } {
    True as whole if whole -> case Cv1([7], True) {
      Cv1([], True) -> []
      Cv1(_, b) if !b || b -> [0, 7]
      a -> [3, 7]
    }
    item -> case Cv1([2], False) {
      _ -> [3]
      Cv1([0, a, ..] as whole, False) -> fn(v2) { [100, 100] }(1.5)
    }
  }
}

pub fn main() {
  echo "bc"
  echo case "constructor" <> "a", "ab" {
    "res" <> _, "a" <> rest if rest == "ab" && rest == "res" -> fn(v3) { fn(v4, v5) { [10] }(True, "a") }(3.14)
    "ab", _ -> []
    "b", _ -> f1("abc", #("bc", 1))
    v6, v7 -> []
  }
  echo case limit_value, 7 - 42 {
    _, z -> True
    10.0, 6 -> !{
      fn(v8) { True }("ab")
    }
  }
}
