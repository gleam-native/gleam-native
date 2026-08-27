pub const golden_value: Bool = True

pub type V0 {
  Error(value: String, inner: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(item: V0, s: Bool, n: List(Int)) -> String {
"a"
}

fn f1(v1: Bool, v2: #(Int, String), rest: #(Float, Float)) -> String {
fn(v3) { case fn(v4, v5) { Error("constructor", "x") }(3, 100.0) {
    Error(b, _) -> fn(v6) { b }(0)
    v7 -> "a"
    Error(inner, _) -> v3
  } }("ab")
}

pub fn main() {
  let constructor = {
    let v = [100]
    let m = fn(v8, v9) { [1] }(10.0, 1.0)
    {
      let n = 100.0
      v
    }
  }
  let constructor = {
    10 * 4
  } % 2
  echo "x"
  echo {
    case constructor {
      9 | 0 -> 10.0
      b -> fn(v10, v11) { 0.25 }(10, 100)
    }
  } == {
    {
      100.0
    } +. {
      fn(v12) { 1.0 }("b")
    }
  }
  echo case constructor, Error("a", "res") {
    5, _ -> fn(v13) { {
      let n = [1]
      let class = v13
      True
    } }(2.0)
    7, Error("abc", _) -> case #(2.0, True), Error("b", "b") {
      #(_, False), Error(_, "ab" <> _) -> golden_value
      #(delete, True), Error("data", golden_value) if delete <=. 1.5 -> {
        let golden_value = golden_value
        let new = constructor
        True
      }
      #(3.14 as whole, _), Error("bc" <> _, "ab") -> constructor != constructor
      _, _ -> False || golden_value
    }
    4, constructor -> golden_value
    v14, _ -> case Error("data", "abc") |> default({
        let x = golden_value
        let constructor = 100.0
        x
      }, fn(v15) { [2, 42] }(42)), fn(v16, v17) { 0.0 }(7, 0) {
      prototype, _ -> False
      "" <> rest, 0.5 -> False && golden_value
    }
  }
  echo golden_value
}
