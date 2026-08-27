pub type Record {
  Cv0(value: String, inner: Bool)
  Cv1
  Cv2(List(Int))
}

pub type V3 {
  Number
}

pub type Symbol {
  Cv4
}

fn f0(v5: #(Int, Bool)) -> String {
case #(2.0, "b") {
    #(0.5, "abc") -> "b"
    #(_, "a" <> rest) as whole -> {
      fn(v6) { "b" }("abc")
    } <> {
      "constructor" <> "data"
    }
    _ -> {
      let v5 = 100
      "x"
    }
  }
}

fn f1(v7: #(Float, Int)) -> List(Int) {
case Cv4, <<"b":utf8>> {
    v8, <<"x":utf8>> -> {
      let new = [7, 5]
      let m = "x"
      {
        let v = 5
        let new = 2.0
        []
      }
    }
    Cv4, <<_:big-unsigned-1, "a":utf8>> -> {
      let v7 = True
      [42, 42]
    }
    Cv4, _ -> [10, 1]
    _, _ -> case fn(v9, v10) { 2 }(True, True) {
      b -> fn(v11, v12) { [2] }("res", 2)
      3 -> [4, 5]
      _ -> [1]
    }
  }
}

pub fn main() {
  let value = False
  echo 3.14
  echo case 100.0 {
    constructor -> case fn(v13, v14) { 1 }(0.1, True), Cv1 {
      3, v15 -> fn(v16) { 10 }("abc")
      item, Cv0("constructor" <> _, v17) -> fn(v18, v19) { item }(0.1, 1.0)
      _, _ -> 100
    }
    a -> 3
  }
  echo case #(2.0, 7) |> f1(), {
      let arguments = 10
      let prototype = False
      #(10.0, True)
    } {
    [1, 0, ..], #(1.5 as whole, _) -> "res"
    [8, 6, ..], #(100.0, _) -> f0(#(5, False))
    v20, _ -> f0(#(7, True))
  }
  echo 5
}
