pub type V0 {
  Cv1(value: List(Int))
  Cv2(Int)
  Cv3(Float, List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(y: Float, default: List(Int)) -> Bool {
{
    {
      let z = default
      "ab"
    }
  } != {
    {
      fn(v4) { "data" }(False)
    } <> {
      "b" <> "ab"
    }
  }
}

fn f1(default: String, z: #(Int, Int), v5: String) -> List(Int) {
{
    let length = 1
    []
  }
}

fn f2(v6: V0, v7: Int, v8: Int) -> String {
case False {
    True | True -> "data" <> {
      {
        let n = "abc"
        let m = "a"
        m
      }
    }
    _ | True -> case fn(v9, v10) { 0 }("", "ab") {
      _ | 2 -> ""
      9 -> "abc" <> "constructor"
    }
  }
}

pub fn main() {
  echo case fn(v11, v12) { "x" }(0.0, False) {
    "b" -> case "bc" <> "", "bc" <> "constructor" {
      "res", "a" <> rest as whole -> f1(rest, #(10, 4), "abc")
      "data" as whole, "constructor" as it -> it |> f1(#(5, 3), "x" <> whole)
      _, _ -> [10]
    }
    b | "b" <> b -> fn(v13, v14) { [] }(100.0, 10)
    constructor -> []
  }
  echo False
  echo case [] |> walk(10), "x" {
    _, "abc" as whole -> {
      {
        let value = [7, 4]
        let value = whole
        10
      }
    } != {
      fn(v15, v16) { 10 }(10, 2)
    }
    _, "constructor" <> _ -> True
    v17, _ -> {
      let self_ = f2(Cv1([3]), 5, v17)
      let acc = 1.0
      {
        let arguments = v17
        True
      }
    }
  }
  echo case fn(v18, v19) { Cv2(7) }(False, True) {
    item -> walk("a" |> f1(#(42, 100), "bc"), 10 + 2)
    Cv2(6) -> walk({
      let delete = [7, 3]
      delete
    }, walk([42, 7], 100))
  }
}
