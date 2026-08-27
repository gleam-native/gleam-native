pub const pi_value: Int = 100

pub type Object {
  Cv0(value: String, inner: String)
  Error(value: String)
}

pub type V1 {
  Cv2(Float, value: Bool)
  Record
}

pub type V3 {
  Cv4
  Cv5(Float, Bool)
  Cv6
}

fn f0(v7: Int, value: Int) -> List(Int) {
[0]
}

fn f1(v8: Int, v9: V1, constructor: V1) -> Bool {
case 3, fn(v10, v11) { Cv4 }(10, True) {
    _, Cv6 -> {
      {
        0.1
      } -. {
        1.5
      }
    } <=. {
      100.0
    }
    _, Cv4 -> v8 < v8
    _, _ -> {
      {
        let this_ = 42
        10.0
      }
    } <=. {
      1.5
    }
  }
}

fn arguments(v12: Float, z: Int) -> String {
case {
      let z = v12
      "x"
    } {
    "data" <> item -> item
    "abc" <> rest -> "abc"
    _ -> case False {
      v13 -> "constructor" <> "res"
      _ -> fn(v14, v15) { "bc" }(10, 2)
      a -> "a"
    }
  }
}

pub fn main() {
  echo case fn(v16, v17) { True }(2, 100.0) {
    True -> False
    a -> False
    inner -> case "a" {
      "constructor" <> item if item != "data" && item != "constructor" -> {
        let z = 1.0
        let item = [10, 100]
        inner
      }
      v18 -> pi_value != 1
      "res" as whole -> {
        let inner = 1
        True
      }
    }
  }
}
