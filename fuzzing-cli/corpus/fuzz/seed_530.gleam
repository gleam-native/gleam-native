pub const seed_value: Bool = True
pub const golden_value: String = "b"

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v3: String, v4: Int, delete: Int) -> List(Int) {
[4, 4]
}

fn f1(class: V0, v5: Float) -> Float {
v5
}

pub fn main() {
  let y = []
  echo case 1.5 {
    _ -> {
      let default = golden_value <> golden_value
      let default = 100.0
      seed_value
    }
    1.0 -> case walk([3], 2) {
      0 | 6 -> True
      7 -> seed_value
      8 | 5 -> seed_value
      v6 -> True
    }
    v7 -> {
      y |> walk(3 + 4)
    } <= 7
  }
  echo case golden_value <> golden_value {
    a | "data" <> a -> "abc"
    _ | "abc" -> case f1(Cv2, 0.0), 3 {
      v, 4 -> golden_value
      100.0, 5 -> "res"
      _, _ -> {
        let this_ = [100, 0]
        golden_value
      }
    }
    "res" <> rest | "ab" <> rest -> {
      let z = walk(y, 2)
      let v = 100 <= z
      "x"
    }
  }
  echo fn(v8) { case v8 {
    b -> walk([], v8)
    v9 -> [7, 1] |> walk({
      let constructor = 0.5
      v8
    })
    item -> 2
  } }(4)
  echo y
}
