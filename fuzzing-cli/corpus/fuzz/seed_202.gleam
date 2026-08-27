pub const pi_value: Bool = False
pub const seed_value: Int = 100

pub type Symbol {
  Cv0(value: String, inner: Int)
  Cv1
  Cv2(List(Int))
}

fn f0(v3: Int, x: #(Bool, Bool)) -> String {
case 100 {
    v4 -> {
      fn(v5, v6) { "res" }(0, 1.0)
    } <> "abc"
    item -> {
      let v = "data" <> "abc"
      let x = 42
      "constructor"
    }
    a -> {
      fn(v7, v8) { "res" }(3.14, True)
    } <> "res"
  }
}

pub fn main() {
  let this_ = case Cv1, 3 + 0 {
    Cv2([]), 2 -> fn(v9) { "bc" }(2.0)
    seed_value, item -> "abc"
    _, _ -> "ab" <> "data"
  }
  echo fn(v10, v11) { [] }(True, 10.0)
  echo case {
      let pi_value = "ab"
      let seed_value = True
      Cv2([])
    } {
    pi_value -> seed_value |> f0(#(True, False))
    Cv0("abc" <> _, this_) -> f0(100 + this_, #(False, True))
    Cv0("constructor" <> _, 2) -> this_
  }
  echo []
}
