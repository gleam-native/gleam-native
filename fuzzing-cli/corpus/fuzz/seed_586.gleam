pub const tag_value: Float = 1.5
pub const euler_value: Bool = True

pub type V0 {
  Error(value: String, inner: Bool)
  Cv1
  Some
}

pub type Symbol {
  Cv2
  Record
  Cv3(Int)
}

pub type V4 {
  Cv5(List(Int))
  Cv6(value: Int)
  Cv7
}

fn f0(pair: List(Int), v8: Int, v9: Int) -> String {
"b"
}

fn new(m: Int, default: Bool, v10: Int) -> List(Int) {
[]
}

fn extends(length: Bool, new: Float, default: Int) -> Bool {
case default - default, default {
    _, v11 -> case #(42, 10) {
      inner -> length
      #(5, 0) as whole -> False
    }
    9, 8 -> case fn(v12, v13) { "res" }(True, 0), Cv6(100) {
      "bc" <> rest, Cv7 -> False
      "data", Cv7 -> True
      v14, v15 -> length
    }
  }
}

pub fn main() {
  let euler_value = "bc" <> f0([], 5, 0)
  echo euler_value
  echo case fn(v16, v17) { Error("bc", False) }("data", "bc") {
    _ -> 1
    _ -> fn(v18, v19) { v18 }(10, True)
    _ -> case {
        let l = 5
        Error("bc", True)
      }, fn(v20) { "res" }(False) {
      Cv1, self_ if self_ == "b" && self_ == "constructor" -> 100
      Cv1, "" <> rest -> 2
      Error("bc" as whole, _), _ -> 7 * 3
      _, _ -> fn(v21) { 3 }("bc")
    }
  }
}
