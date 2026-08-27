pub const pi_value: Float = 1.0

pub type Number {
  Cv0(value: String, inner: Bool)
  Cv1(Int)
  Cv2(String, Bool)
}

pub type V3 {
  Error(value: Float, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: Int, v4: V3, v5: Int) -> String {
"x" <> "x"
}

fn f1(v6: List(Int), v7: Int, v8: Float) -> List(Int) {
case [], fn(v9) { 3 }(100.0) {
    [5, ..rest], 6 -> v6
    [3, h, ..], 8 -> [5, 2]
    v10, _ -> {
      let v7 = {
        let v = "b"
        let v8 = v7
        v
      }
      let pair = 3.14
      v6
    }
  }
}

pub fn main() {
  let length = case [] {
    [] as whole -> True
    [7] -> {
      let rest = 5
      True
    }
    [1, ..rest] -> True
    v11 -> 5 <= 1
  }
  let this_ = 10.0
  echo {
    let length = 0.25
    let pi_value = fn(v12) { {
      1.5
    } *. pi_value }(0.5)
    []
  }
  echo "b"
  echo walk([10], 10 % 6) < {
    1 - 5
  }
}
