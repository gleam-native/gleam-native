pub const euler_value: Int = 0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(class: Bool) -> Int {
[100, 4] |> walk(10 - 7)
}

fn f1(length: Float, pair: V0) -> Float {
length
}

fn f2(v3: V0, v4: Bool) -> Float {
10.0
}

pub fn main() {
  let v = True
  echo 0.1
  echo 100.0
}
