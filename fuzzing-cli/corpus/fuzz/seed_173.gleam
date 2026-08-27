pub const seed_value: String = ""
pub const golden_value: String = "bc"

pub type Promise {
  Cv0(value: String, inner: Float)
  Cv1(List(Int), value: List(Int))
}

pub type V2 {
  Ok(Bool)
}

fn f0(m: #(List(Int), String), v3: Int) -> String {
"b" <> {
    {
      let y = fn(v4, v5) { [] }(2.0, True)
      let v3 = fn(v6, v7) { v6 }("abc", "bc")
      v3 <> "b"
    }
  }
}

fn f1(v8: Float, v9: V2, value: Float) -> String {
case "bc", fn(v10, v11) { True }("abc", 0.5) {
    "constructor", True as whole -> {
      let whole = fn(v12, v13) { [] }("res", 0.5)
      "a"
    }
    "ab", False -> "abc"
    v14, _ -> v14
  }
}

fn default(v15: Float, v16: Int, v17: V2) -> Int {
v16
}

pub fn main() {
  let seed_value = fn(v18, v19) { [] }(0.1, "abc")
  let seed_value = {
    let m = f1(10.0, Ok(False), 0.0)
    True
  }
  echo {
    let item = golden_value
    let golden_value = seed_value
    5
  }
  echo case 1 + 2, [] {
    1, [9] -> f1(fn(v20) { 100.0 }(0.0), Ok(True), {
      let this_ = ""
      1.0
    })
    0 as whole, [] -> {
      #([100, 2], "a") |> f0(100)
    } <> "b"
    9, [_, golden_value, ..] -> case "bc", f0(#([], "res"), 10) {
      _, "constructor" as whole -> "ab"
      golden_value, "res" as whole -> {
        let z = 2.0
        "x"
      }
      v21, v22 -> "x"
    }
    v23, _ -> ""
  }
  echo []
}
