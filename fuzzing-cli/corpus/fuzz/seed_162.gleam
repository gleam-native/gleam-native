pub const limit_value: Int = 4

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Number(value: Float)
  Ok(Float, value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(length: String) -> Int {
{
    case False || True {
      item -> 5 + 100
      True -> 2
      False | True -> fn(v2, v3) { 3 }(10.0, True)
    }
  } - 1
}

fn delete(s: String) -> String {
s <> {
    {
      "" <> s
    } <> {
      "constructor" <> s
    }
  }
}

pub fn main() {
  let y = "b"
  let value = y <> {
    "x" <> y
  }
  echo "x" <> {
    {
      value <> "res"
    } |> delete()
  }
}
