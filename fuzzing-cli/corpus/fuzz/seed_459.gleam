pub const pi_value: Float = 100.0
pub const limit_value: Bool = False

pub type Promise {
  Cv0(value: String, inner: Bool)
}

pub type V1 {
  Cv2
}

fn arguments(v3: Int, delete: Int) -> List(Int) {
case Cv0("bc", True) {
    Cv0("" <> rest, True) if rest == "bc" && rest != "b" -> []
    Cv0("a" <> _, True) -> [2, 4]
    _ -> [100, 5]
  }
}

fn yield(v4: Bool, v5: String) -> Bool {
case Cv2, v5 {
    Cv2, "x" <> rest -> {
      fn(v6, v7) { 0.0 }(0.1, 0.5)
    } >=. {
      0.25
    }
    Cv2, "a" -> False
    _, "constructor" -> True
    v8, _ -> v4
  }
}

pub fn main() {
  let x = case fn(v9, v10) { Cv0("ab", True) }(0, "data"), "bc" != "x" {
    Cv0("ab" as whole, _) as it, pi_value -> fn(v11) { [10, 7] }(2.0)
    Cv0("b" <> rest, _), _ -> [10]
    v12, _ -> [100]
  }
  let length = case pi_value {
    0.5 -> arguments(1, 5)
    b -> x
  }
  echo "res"
  echo case Cv2 {
    inner -> case fn(v13, v14) { v13 }(7, False) {
      v15 -> 0.25
      0 | 1 -> 3.14
    }
    Cv2 -> 1.0
    Cv2 | Cv2 -> 0.0
  }
  echo pi_value
}
