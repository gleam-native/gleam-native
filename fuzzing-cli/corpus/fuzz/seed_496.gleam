pub const golden_value: Int = 100

pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Number
  Cv4
}

pub type V5 {
  None(value: Int, inner: Bool)
  Cv6(value: Float)
}

fn f0(v7: Int) -> Float {
case "a" <> "a", Cv1 {
    _, Cv2 -> fn(v8) { {
      10.0
    } +. {
      2.0
    } }(True)
    v7, Cv2 -> 1.0
    _, v9 -> case <<7:8, 3:8>>, [4, 3] {
      <<_:utf8>>, [] as whole -> {
        let v = 3.14
        let prototype = "ab"
        v
      }
      <<_:8, _:8>> as whole, [6, ..rest] -> {
        100.0
      } /. {
        0.5
      }
      _, _ -> 10.0
    }
  }
}

pub fn main() {
  echo {
    case f0(4) {
      100.0 -> golden_value - golden_value
      a -> golden_value
    }
  } - {
    case fn(v10, v11) { Cv4 }("b", "") {
      Cv4 -> golden_value
      a -> 10
      item -> 42 - golden_value
    }
  }
  echo True
  echo f0(golden_value + 5) +. {
    {
      {
        100.0
      } /. {
        0.5
      }
    } +. {
      {
        let golden_value = "a"
        2.0
      }
    }
  }
}
