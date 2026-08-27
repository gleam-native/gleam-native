pub type Object {
  Cv0(value: String, inner: Bool)
  Cv1(Float)
  Cv2(List(Int))
}

fn default(v3: Bool) -> List(Int) {
[0, 3]
}

pub fn main() {
  let n = case {
      let l = True
      let constructor = "bc"
      Cv1(0.0)
    } {
    a -> [100, 10]
    Cv1(_) | Cv0(_, _) -> False |> default()
    Cv1(item) -> []
  }
  echo fn(v4) { 100.0 }("")
  echo True |> default()
}
