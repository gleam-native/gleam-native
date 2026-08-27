pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(String, List(Int))
  Cv3(Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(x: V0, m: Bool, v4: Int) -> Int {
3
}

pub fn main() {
  let delete = False
  echo fn(v5, v6) { case 2.0 {
    v7 -> {
      let delete = v5
      "res"
    }
    b -> ""
  } }(0.25, "ab")
}
