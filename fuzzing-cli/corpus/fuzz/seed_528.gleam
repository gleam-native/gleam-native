pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(value: Float)
  Cv4(value: Int, inner: Bool)
  Cv5(Float)
}

pub type Object {
  Some(String, Float)
  None(value: List(Int), inner: String)
}

fn f0(item: Float, self_: Bool) -> Float {
fn(v6, v7) { 10.0 }(3, False)
}

pub fn main() {
  echo case <<1:4, "ab":utf8>> {
    <<"":utf8>> -> case 0 > 3, 10.0 {
      v8, 0.5 -> [42]
      True, 10.0 -> {
        let item = [2, 10]
        item
      }
      v9, 1.0 -> [2]
      _, _ -> {
        let delete = 0.5
        [0]
      }
    }
    _ -> [2, 1]
  }
  echo "constructor"
  echo {
    {
      "data" <> "b"
    } <> "bc"
  } == {
    "constructor" <> "res"
  }
  echo []
}
