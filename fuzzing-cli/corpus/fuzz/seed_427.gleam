pub const limit_value: String = "x"

pub type Symbol {
  Record
  Cv0(value: Bool, inner: Bool)
  None(String)
}

pub type V1 {
  Cv2(value: Bool)
  Cv3(value: Float)
  Cv4(Float)
}

pub type V5 {
  Cv6(value: Float, inner: Bool)
}

fn f0(m: String, v7: Bool) -> Bool {
True
}

pub fn main() {
  let rest = case {
      let delete = 0.25
      Cv2(False)
    } {
    Cv3(3.14) | Cv4(_) -> {
      100.0
    } *. {
      0.5
    }
    _ -> 0.0
    Cv4(1.0) | Cv4(_) -> 0.1
  }
  echo case rest, 5 + 1 {
    0.1, 8 as whole -> 0.25
    limit_value, 5 -> 1.0
    0.1, _ -> 0.25
    v8, _ -> v8
  }
  echo case Cv2(True) {
    Cv2(new) -> 2.0
    _ -> {
      0.5
    } /. {
      0.5
    }
    Cv3(2.0) | Cv3(_) -> {
      {
        let rest = 3
        let rest = True
        2.0
      }
    } *. {
      100.0
    }
  }
  echo limit_value <> "abc"
  echo case <<"ab":utf8, 42:8>> {
    <<_:utf8, 3:8>> -> 7
    <<4:8, "constructor":utf8>> -> {
      let arguments = limit_value
      5
    }
    _ -> 100
  }
}
