pub const seed_value: String = "b"

pub type V0 {
  Ok(value: String, inner: List(Int))
  Cv1(value: Int, inner: List(Int))
}

fn f0(class: Int) -> Bool {
True
}

fn f1(pair: V0) -> Int {
5
}

fn constructor(delete: #(Bool, List(Int)), pair: List(Int), v2: #(Float, List(Int))) -> String {
"bc"
}

pub fn main() {
  let seed_value = fn(v3) { [] }(10.0)
  let acc = case fn(v4, v5) { 4 }("", 3.14) {
    _ -> "a"
    a -> "bc"
  }
  echo {
    {
      let acc = 100.0
      let m = constructor(#(True, [5, 5]), seed_value, #(0.0, [7]))
      constructor(#(False, [2, 3]), seed_value, #(1.0, [100]))
    }
  } <> constructor(#(False, [42, 1]), fn(v6, v7) { [0] }(0.0, 10), {
    let default = True
    let acc = acc
    #(10.0, [100, 42])
  })
  echo fn(v8, v9) { case "x" {
    _ | "constructor" -> v9
    "bc" -> False
    "res" -> !v9
  } }(0.5, True)
}
