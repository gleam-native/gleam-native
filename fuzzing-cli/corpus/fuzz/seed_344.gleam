pub const pi_value: String = "data"
pub const seed_value: Int = 1

pub type V0 {
  Some(value: String, inner: String)
}

fn f0(v1: Bool, m: Int, v2: #(Bool, Float)) -> Bool {
v1
}

fn f1(default: V0, arguments: V0, v: Float) -> Float {
v
}

fn static(v3: Bool) -> Float {
f1(fn(v4) { fn(v5, v6) { Some("b", "a") }(True, "a") }(False), Some("res", "data"), case "b" <> "" {
    item | "abc" <> item -> Some("b", "") |> f1(Some("data", ""), fn(v7) { v7 }(0.1))
    constructor -> 2.0
    constructor -> fn(v8) { 1.5 }(2)
  })
}

pub fn main() {
  let pi_value = 100
  let seed_value = {
    Some("ab", "b") |> f1({
      let class = 1
      Some("b", "res")
    }, fn(v9) { v9 }(100.0))
  } >. {
    0.0
  }
  echo "a"
  echo 4
  echo case fn(v10, v11) { v10 }("data", 3), [5] {
    "abc", [3] -> {
      {
        0.5
      } -. {
        1.0
      }
    } -. {
      {
        0.1
      } +. {
        10.0
      }
    }
    "data" <> rest, [_] as whole -> fn(v12) { fn(v13, v14) { 0.1 }(True, 0.0) }(True)
    v15, _ -> fn(v16) { {
      let pi_value = v16
      let default = ""
      10.0
    } }(1.5)
  }
}
