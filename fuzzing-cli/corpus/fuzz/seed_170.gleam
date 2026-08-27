pub type V0 {
  Cv1
  Cv2
  Cv3(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(pair: #(List(Int), Int), new: List(Int), v4: Float) -> Float {
v4
}

fn default(v: Bool) -> Float {
case [] {
    [7, _, ..] -> 0.25
    [] -> 10.0
    _ -> 0.0
  }
}

pub fn main() {
  let class = case fn(v5) { Cv2 }(10.0) {
    Cv1 -> {
      let constructor = 4
      []
    }
    _ | Cv1 -> []
  }
  echo [10]
}
