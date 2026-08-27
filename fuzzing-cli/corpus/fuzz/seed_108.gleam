pub const golden_value: Int = 100

pub type V0 {
  Error(value: String, inner: List(Int))
  Cv1
  Cv2(value: Float)
}

pub type Symbol {
  Cv3
  Record(String, String)
  Cv4(Int, value: String)
}

pub type V5 {
  Cv6(Float)
  Cv7(Bool)
}

fn static(arguments: Bool, this_: Int, v8: Bool) -> String {
"b"
}

pub fn main() {
  let value = case Cv1, {
      let golden_value = 4
      1
    } {
    Cv2(0.5), 5 -> {
      let acc = []
      let golden_value = "bc"
      0.1
    }
    self_, 9 as whole -> {
      0.5
    } -. {
      0.5
    }
    Cv2(0.0), _ -> 2.0
    _, v9 -> fn(v10, v11) { 1.0 }(100, 0.25)
  }
  let length = [3]
  echo case value {
    a -> case Cv7(False) {
      b -> True
      constructor -> False
      Cv6(2.0) | Cv6(_) -> True
    }
    1.5 | 0.5 -> case fn(v12) { length }(5), fn(v13) { Error("a", [7]) }(1.0) {
      [], Cv2(1.5) -> !True
      [5, 3, ..] as whole, Cv1 -> True
      [7], Cv2(1.5) -> fn(v14) { v14 }(False)
      v15, _ -> False
    }
    b -> case 10.0 {
      2.0 -> False
      inner -> golden_value > 4
      b -> 4 >= 0
    }
  }
  echo case True && True {
    b -> {
      {
        0.5
      } +. value
    } /. {
      0.5
    }
    v16 -> value
    _ -> case Record("bc", "a") {
      inner -> {
        3.14
      } -. {
        0.0
      }
      _ -> value
      Cv3 -> {
        let length = length
        value
      }
    }
  }
  echo {
    {
      {
        let rest = length
        "bc"
      }
    } <> static(True, 5, False)
  } <> {
    {
      "b" <> "res"
    } <> "x"
  }
}
