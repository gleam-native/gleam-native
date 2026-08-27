pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int, inner: Float)
  Cv3(List(Int), Bool)
}

pub type V4 {
  Error(value: List(Int))
  Cv5(value: Int)
}

fn f0(value: Float) -> Int {
0 - 42
}

fn f1(v6: V4, y: String) -> String {
""
}

fn f2(v7: Bool, length: Int) -> List(Int) {
fn(v8) { case "constructor" <> "x", {
      let v8 = length
      let value = [3]
      [1]
    } {
    "constructor", [] -> fn(v9) { [] }(3)
    _, [8, _, ..] -> []
    prototype, [] -> []
    _, v10 -> [3]
  } }(2.0)
}

pub fn main() {
  let s = 3.14
  let s = 100
  echo "a" <> "data"
  echo True
  echo 100
  echo case s {
    _ -> [5, 100]
    9 | 2 -> case fn(v11) { "" }(True) {
      item | "a" <> item -> [7, 42]
      a -> f2(False, 10)
      _ -> True |> f2({
        let n = s
        5
      })
    }
  }
}
