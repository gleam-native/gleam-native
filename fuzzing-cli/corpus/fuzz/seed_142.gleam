pub const golden_value: Int = 0

pub type Number {
  Cv0(value: String, inner: String)
  Ok
}

pub type V1 {
  Cv2
  Record(value: List(Int), inner: Bool)
  Cv3(Float, String)
}

fn f0(arguments: Float, v4: Number, class: Float) -> Float {
fn(v5, v6) { case {
      let m = "constructor"
      []
    }, fn(v7) { "bc" }("res") {
    [], "b" -> {
      1.0
    } /. {
      0.5
    }
    [8, _, ..] as whole, "res" -> 1.5
    _, v8 -> arguments +. {
      0.1
    }
  } }(10.0, True)
}

pub fn main() {
  let class = "res"
  echo {
    case True, class {
      _, "bc" -> 4
      False, _ -> 7
      prototype, "b" -> {
        let class = [100, 7]
        let v = 0.5
        golden_value
      }
      v9, _ -> 42 * 100
    }
  } < golden_value
  echo [2, 2]
  echo {
    class <> class
  } <> class
}
