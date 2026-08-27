pub const euler_value: String = "constructor"

pub type Symbol {
  Cv0(value: String, inner: Int)
  Cv1(value: Bool)
}

fn f0(l: String, v2: Int) -> String {
"data" <> {
    l <> "res"
  }
}

pub fn main() {
  echo True
  echo 0.25
  echo {
    {
      euler_value |> f0(fn(v3) { 7 }(0.25))
    } <> "res"
  } <> {
    f0("", 4) |> f0(5 + 1)
  }
}
