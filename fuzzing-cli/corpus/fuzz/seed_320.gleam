pub const tag_value: Int = 3
pub const golden_value: Float = 0.25
pub const euler_value: Bool = False

pub type V0 {
  Some(value: String, inner: String)
  Cv1
  Cv2(Float, value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v3: Int) -> String {
"abc"
}

fn f1(v4: Int) -> Int {
walk([], {
    fn(v5, v6) { 3 }(True, 3)
  } - {
    v4 + 4
  })
}

pub fn main() {
  let this_ = {
    let length = False
    let length = golden_value
    static(4)
  }
  echo case [] |> walk(tag_value) {
    0 -> f1(1) < {
      fn(v7) { tag_value }(100.0)
    }
    _ -> case {
        let value = "a"
        2
      } {
      b -> {
        0.5
      } <=. {
        0.0
      }
      _ -> False
    }
    tag_value -> case fn(v8) { 4 }(1), {
        let tag_value = euler_value
        let x = 10.0
        Cv2(3.14, 2)
      } {
      v9, Cv1 if v9 > 9 && v9 > 9 -> euler_value
      v10, _ -> 0 >= v10
      0 as whole, Cv2(this_, _) as it -> {
        let value = 2.0
        euler_value
      }
    }
  }
}
