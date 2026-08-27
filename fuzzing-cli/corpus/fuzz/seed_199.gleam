pub const tag_value: Int = 4
pub const euler_value: Bool = False
pub const limit_value: Bool = True

pub type V0 {
  None(value: String, inner: String)
}

pub type V1 {
  Cv2
  Cv3(value: Int)
  Cv4(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(rest: Bool, v5: V1) -> String {
"res"
}

fn export(v6: Float, v: Int) -> Int {
{
    case "" <> "a" {
      "bc" -> fn(v7, v8) { 2 }(0.5, "x")
      "constructor" -> spin(v, v)
      _ -> v
    }
  } + 1
}

pub fn main() {
  let arguments = euler_value
  let x = True
  echo case fn(v9, v10) { Cv4(4) }("ab", True) {
    Cv4(_) as whole -> fn(v11, v12) { fn(v13, v14) { v12 }("data", False) }(True, 100)
    Cv4(inner) -> fn(v15, v16) { inner }(0.1, 3)
    Cv4(2) -> 42
    _ -> 0
  }
  echo case 1, <<"b":utf8>> {
    2, <<"bc":utf8, z:little-signed-8>> if z > 0 || z % 2 == 0 -> case Cv3(4) {
      Cv3(5) | Cv2 -> [2, 10]
      _ | Cv2 -> []
    }
    acc, _ -> [42, 7]
  }
  echo tag_value + {
    fn(v17, v18) { {
      0.25
    } |> export(spin(tag_value, 0)) }(1.0, 3)
  }
  echo {
    delete(False, Cv4(7)) <> {
      "bc" <> "x"
    }
  } <> {
    case fn(v19) { [] }(100), 100.0 {
      [a, 8, ..], 1.0 if a == 0 -> "abc"
      [], 0.25 as whole -> delete(limit_value, Cv3(1))
      _, v20 -> delete(True, Cv2)
    }
  }
}
