pub const euler_value: String = "constructor"
pub const pi_value: Bool = True
pub const golden_value: String = ""

pub type Record {
  Cv0(value: String, inner: List(Int))
  Cv1(value: Int)
  Cv2
}

pub type V3 {
  Cv4
  Cv5(Float)
}

pub type V6 {
  Cv7(Bool)
}

fn f0(default: Int) -> Bool {
case {
      let value = 3.14
      let item = 100.0
      [4]
    }, Cv2 {
    [6], Cv1(5) -> {
      10 - 42
    } != default
    [], Cv0(m, [8, _, ..]) -> False
    v8, v9 -> {
      "data" <> "ab"
    } != {
      {
        let x = v8
        let n = [3, 1]
        "bc"
      }
    }
  }
}

fn f1(v10: Int, v11: Int, v12: Float) -> List(Int) {
[5, 7]
}

pub fn main() {
  let x = 10 |> f1(5 - 10, 100.0)
  echo {
    let s = {
      fn(v13) { 1.5 }(42)
    } -. {
      {
        1.0
      } /. {
        2.0
      }
    }
    case "data" <> euler_value, Cv5(0.0) {
      _, Cv4 -> {
        let s = "ab"
        0.0
      }
      "x", Cv4 as whole -> 3.14
      v14, _ -> 1.5
    }
  }
  echo {
    case 0 {
      5 | 8 -> {
        let golden_value = 7
        let golden_value = False
        3.14
      }
      6 | 4 -> 3.14
      constructor -> fn(v15) { 0.1 }("constructor")
    }
  } == {
    0.0
  }
  echo {
    case 4 + 1, #(2, [5, 4]) {
      _, #(7, [_]) -> 2
      delete, #(_, [x, ..rest]) -> x
      _, _ -> fn(v16) { 10 }("constructor")
    }
  } |> f0()
}
