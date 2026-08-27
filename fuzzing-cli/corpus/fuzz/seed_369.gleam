pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(Float, value: Bool)
}

fn f0(l: List(Int)) -> Float {
case Cv1([], 1) {
    Cv2(_, b) -> fn(v3) { 0.0 }(3.14)
    Cv2(inner, _) -> case Cv1([10, 7], 1), #("ab", 100) {
      Cv2(0.5, False), #("data" <> rest, 4) -> {
        0.0
      } -. inner
      Cv1([8, ..rest], _), #(_, 3) -> inner +. {
        2.0
      }
      _, v4 -> inner *. inner
    }
    _ -> case <<"a":utf8>> {
      <<_:utf8, _:little-unsigned-8, _:utf8>> -> 1.0
      <<"a":utf8>> -> {
        100.0
      } +. {
        2.0
      }
      _ -> 0.1
    }
  }
}

fn f1(prototype: Int, l: Int, class: String) -> String {
"data"
}

fn f2(v5: List(Int), v6: Float, l: Bool) -> Float {
{
    {
      100.0
    } -. {
      v6 +. {
        0.0
      }
    }
  } -. {
    {
      let n = True
      let constructor = 0.25
      v6
    }
  }
}

pub fn main() {
  let delete = {
    "x" <> "constructor"
  } <> {
    "res" <> "constructor"
  }
  echo !False
}
