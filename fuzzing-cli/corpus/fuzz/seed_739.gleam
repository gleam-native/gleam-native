pub const golden_value: Float = 0.25

pub type V0 {
  Record(value: String, inner: Int)
  Cv1(Int)
  Ok
}

fn f0(prototype: Float, v2: #(List(Int), String), v3: Float) -> String {
"a" <> {
    fn(v4) { "res" }(0.0)
  }
}

fn static(s: V0, v5: String, v6: List(Int)) -> List(Int) {
[0, 100]
}

fn f2(v7: List(Int)) -> Int {
1
}

pub fn main() {
  let golden_value = case Cv1(10), "abc" {
    Ok, _ -> [10, 10]
    Record(_, constructor), v8 if constructor == 2 && v8 == "data" -> []
    _, "res" <> _ -> Ok |> static(fn(v9, v10) { v9 }("x", 4), [])
    v11, v12 -> static(Ok, "ab", [2])
  }
  let golden_value = golden_value
  echo 3
  echo golden_value
  echo {
    let value = True
    let value = "res"
    case 7, Cv1(1) {
      3, l -> static(l, "abc", [])
      9, Ok -> [5]
      v13, _ -> [3]
    }
  }
}
