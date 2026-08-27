pub const seed_value: Int = 3
pub const tag_value: Bool = False

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn static(s: V0, n: Bool) -> List(Int) {
case {
      let this_ = []
      s
    } {
    item -> [0]
    Cv1([] as whole, 9) -> case s, "a" {
      _, _ -> whole
      Cv1([_], 3), "ab" as whole -> [1, 4]
      Cv1([_, ..rest], _), "bc" <> tail as whole -> [5]
    }
  }
}

fn new(delete: #(Float, Int), item: List(Int)) -> Bool {
False
}

pub fn main() {
  let length = False
  let seed_value = case "" {
    b -> True || length
    "abc" -> True || tag_value
    "b" | "constructor" <> _ -> True
  }
  echo case "ab", seed_value {
    _, True -> {
      fn(v2) { 10 }(2)
    } * {
      {
        let pair = "b"
        let x = seed_value
        2
      }
    }
    "x" as whole, False -> case "abc" {
      "ab" <> rest -> 0
      "x" <> rest if rest != "b" || rest == "a" -> 42 - 7
      "data" <> constructor | "" <> constructor -> 100 - 42
      _ -> 5 - 42
    }
    v3, _ -> 5
  }
}
