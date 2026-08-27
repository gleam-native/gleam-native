pub const seed_value: Int = 1
pub const limit_value: Bool = True

pub type Promise {
  Cv0(value: String, inner: Float)
  Cv1(List(Int), value: Bool)
  Cv2(Int, value: Float)
}

pub type V3 {
  Cv4(String, List(Int))
  Cv5
}

fn f0(x: List(Int)) -> Int {
case Cv0("data", 2.0) {
    Cv0(v6, 10.0) -> case Cv4("ab", [5, 5]), "b" <> "ab" {
      Cv4(_, [5]), value -> {
        let x = False
        42
      }
      Cv5, _ -> 7
      _, _ -> 3
    }
    Cv1([], False as whole) -> 3
    _ -> 1
  }
}

fn export(v7: Promise, n: #(String, String), delete: Int) -> Bool {
True
}

pub fn main() {
  echo export(Cv0("b", 0.1), #("ab", "data"), {
    let delete = "res" <> "constructor"
    f0([])
  })
}
