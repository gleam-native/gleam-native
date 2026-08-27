pub const euler_value: String = "a"
pub const limit_value: Float = 0.1
pub const golden_value: Bool = False

pub type V0 {
  None(value: String, inner: Float)
  Cv1(value: Bool, inner: String)
}

fn f0(new: Int, v2: Int) -> Int {
{
    let value = case <<"x":utf8>>, "constructor" {
      <<10:8, _:4>>, "ab" -> 1 != new
      <<_:utf8>>, _ -> False
      _, v3 -> True
    }
    let s = {
      let arguments = fn(v4) { 3.14 }(False)
      fn(v5, v6) { True }("res", "")
    }
    case "bc" <> "b", "data" {
      "ab", _ -> new - 5
      "data" <> rest, "ab" <> tail -> v2 % 5
      _, _ -> new
    }
  }
}

pub fn main() {
  echo case None("a", 3.14) {
    Cv1(b, _) if !b || !b -> case [] {
      [0] -> {
        let value = 42
        euler_value
      }
      [x, ..rest] as whole if x == 7 -> euler_value <> euler_value
      [h, ..rest] -> euler_value
      v7 -> {
        let limit_value = golden_value
        "data"
      }
    }
    _ -> case euler_value {
      delete -> {
        let rest = True
        "a"
      }
      item -> "bc" <> item
    }
    None("b" <> rest, 1.0 as whole) -> {
      fn(v8, v9) { rest }("ab", True)
    } <> {
      euler_value <> "constructor"
    }
  }
  echo golden_value
  echo case 1 > 0 {
    _ | True -> case fn(v10, v11) { "abc" }(True, 100.0) {
      "a" -> !True
      "b" <> b -> True || golden_value
      b -> golden_value || golden_value
    }
    _ -> "constructor" == {
      {
        let prototype = 3
        euler_value
      }
    }
  }
}
