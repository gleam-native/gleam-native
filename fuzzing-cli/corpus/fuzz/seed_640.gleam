pub type V0 {
  None(value: String, inner: Float)
  Cv1(String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(s: #(String, List(Int))) -> String {
fn(v2) { "abc" <> {
    "" <> "x"
  } }(0.5)
}

fn new(constructor: Float) -> Bool {
True
}

pub fn main() {
  let v = True
  let v = {
    1.0
  } |> new()
  echo "res" <> "bc"
}
