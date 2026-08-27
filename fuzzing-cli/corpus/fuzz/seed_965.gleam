pub type V0 {
  Number(value: String, inner: List(Int))
  Cv1(Float)
  Cv2(String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: #(Float, List(Int)), v4: V0, n: Int) -> Int {
n % 5
}

fn f1(v5: V0, rest: Float) -> List(Int) {
[]
}

fn delete(this_: Float, arguments: Bool) -> Float {
this_
}

pub fn main() {
  let this_ = {
    10.0
  } -. {
    fn(v6) { v6 }(100.0)
  }
  let l = [3]
  echo case 100 {
    inner -> fn(v7, v8) { False }(3, "res")
    inner -> False
  }
}
