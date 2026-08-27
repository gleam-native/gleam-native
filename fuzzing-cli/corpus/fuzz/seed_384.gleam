pub const tag_value: Float = 0.0
pub const seed_value: Bool = False
pub const pi_value: Bool = True

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Bool)
  Number
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(new: List(Int), item: V0, v3: Int) -> List(Int) {
case 1.0, fn(v4) { 3.14 }(False) {
    1.0 as whole, 3.14 as it -> [5, 10]
    10.0 as whole, 0.5 -> case fn(v5) { Cv1([], 2) }(10), new |> walk(1) {
      Cv2(item), 3 if item || item -> [10]
      Cv2(_), 2 -> []
      v6, _ -> [4]
    }
    v7, 0.25 -> case item {
      Cv1(_, constructor) -> [1]
      _ | Number -> {
        let v3 = new
        let v3 = True
        [10, 2]
      }
    }
    v8, v9 -> new
  }
}

pub fn main() {
  let seed_value = {
    {
      let acc = tag_value
      let seed_value = "ab"
      seed_value
    }
  } <> {
    {
      let x = 42
      "ab"
    }
  }
  let z = case 1.0 {
    _ | 10.0 -> [4]
    0.25 -> [3, 42]
    0.0 | 0.1 -> fn(v10) { [] }(100)
  }
  echo {
    seed_value == {
      {
        let s = 5
        let acc = z
        seed_value
      }
    }
  } && {
    {
      7 - 3
    } == {
      0 + 2
    }
  }
}
