pub const golden_value: Float = 10.0
pub const euler_value: Bool = False

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type V3 {
  Cv4
  Cv5(List(Int), List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: Float, rest: Float, v6: Int) -> List(Int) {
fn(v7) { {
    let delete = {
      let s = []
      let this_ = v6
      42
    }
    [10, 1]
  } }(1.5)
}

fn f1(x: String, v8: Float) -> List(Int) {
[]
}

fn arguments(z: #(Float, Int), x: List(Int), v: #(String, List(Int))) -> String {
case fn(v9, v10) { v10 }(3.14, 2) {
    v11 -> case {
        let l = x
        let s = l
        "data"
      }, {
        let arguments = v11
        [100, 100]
      } {
      "abc" <> rest as whole, [_, 8, ..] as it if rest == "res" || whole == "a" -> "abc" <> "x"
      "b" <> rest, [b, _, ..] as whole if b <= 1 -> {
        let rest = False
        "x"
      }
      v12, [] -> fn(v13, v14) { v12 }(False, False)
      v15, _ -> "abc"
    }
    _ -> "abc"
  }
}

pub fn main() {
  let z = fn(v16, v17) { [42, 1] }("ab", 3.14)
  echo z |> walk({
    let default = "res"
    let golden_value = 2.0
    5
  })
  echo "b"
  echo case 5, #(10, [5, 42]) {
    v18, #(golden_value, [7]) if v18 <= 8 && v18 > 7 -> fn(v19, v20) { 3.14 }(3.14, 1)
    4 as whole, #(v21, [8, ..rest]) -> 0.1
    1, #(_, []) -> golden_value -. {
      {
        let z = golden_value
        let z = euler_value
        golden_value
      }
    }
    v22, v23 -> case fn(v24, v25) { "" }(2.0, False) {
      "data" -> golden_value
      "" <> _ -> fn(v26, v27) { golden_value }(100, "res")
      v28 -> golden_value
    }
  }
}
