pub type V0 {
  Ok(value: String, inner: Bool)
  Cv1(List(Int), value: List(Int))
}

pub type Map {
  Error
  Cv2(value: List(Int), inner: Int)
}

pub type V3 {
  Cv4(value: Int, inner: Int)
  Cv5
}

fn f0(value: String) -> String {
case Cv2([2, 2], 2) {
    Cv2(_, a) if a % 2 == 0 -> value
    Cv2([constructor], _) -> {
      fn(v6) { value }(True)
    } <> value
    v7 -> {
      let y = fn(v8, v9) { [10, 0] }(False, 5)
      let item = []
      value <> value
    }
  }
}

pub fn main() {
  let rest = 3.14
  let delete = f0("a" |> f0())
  echo rest
  echo rest
  echo case <<"data":utf8, "a":utf8, "b":utf8>> {
    <<_:utf8>> -> True
    <<2:8>> -> False
    _ -> {
      fn(v10, v11) { v10 }(0.1, True)
    } == rest
  }
  echo 100
}
