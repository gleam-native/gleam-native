pub const pi_value: Int = 0
pub const euler_value: Float = 100.0

pub type V0 {
  None(value: String, inner: String)
  Cv1
}

pub type V2 {
  Cv3
  Cv4(List(Int))
}

fn export(s: Int) -> Bool {
True
}

fn f1(x: String, v5: String) -> Bool {
case v5 <> x {
    b | "res" <> b -> case fn(v6, v7) { v7 }("abc", True) {
      _ -> fn(v8, v9) { False }(True, True)
      True -> v5 != x
    }
    inner -> True
  }
}

pub fn main() {
  echo "data" <> {
    case euler_value -. {
        0.0
      }, pi_value {
      1.0, 5 -> {
        let l = 100.0
        "data"
      }
      3.14, _ -> "data"
      0.5, 8 -> "b"
      v10, v11 -> "x"
    }
  }
  echo "data"
}
