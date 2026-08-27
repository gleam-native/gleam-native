pub type V0 {
  Cv1
  Cv2
  Cv3(value: Float, inner: Int)
}

pub type V4 {
  Cv5
  Record(Float)
}

pub type V6 {
  Ok(List(Int), Int)
  Cv7(Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v8: V4, l: #(Int, Bool)) -> Float {
{
    let l = {
      let class = 1.5
      let class = fn(v9) { v9 }("bc")
      {
        let acc = 0.1
        0.0
      }
    }
    case 42 - constructor, #("", 1.0) {
      6 as whole, #("a", item) -> 2.0
      8, #("bc" <> _, 0.5 as whole) -> whole
      _, v10 -> fn(v11, v12) { 1.0 }(0.1, "constructor")
    }
  }
}

pub fn main() {
  echo f0(case <<"":utf8, 10:8, "constructor":utf8>> {
    <<class:16, 3:8>> if class > 8 && class <= 7 -> fn(v13) { class }(False)
    <<_:utf8>> -> 5
    <<"a":utf8>> -> fn(v14) { 3 }(True)
    _ -> 3 * 1
  }, Cv5, case Cv5 {
    inner -> {
      let inner = False
      #(42, True)
    }
    Record(_) -> fn(v15, v16) { #(100, True) }(True, 42)
  })
  echo 7
  echo 0.25
  echo {
    {
      "ab" <> "x"
    } <> {
      "x" <> "x"
    }
  } <> {
    {
      {
        let self_ = 5
        let m = 4
        "res"
      }
    } <> "abc"
  }
}
