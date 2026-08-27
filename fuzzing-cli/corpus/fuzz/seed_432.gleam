pub const golden_value: Float = 0.1
pub const euler_value: String = "data"
pub const seed_value: String = ""

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(String, value: Int)
}

fn default(v3: #(Bool, Int)) -> List(Int) {
[]
}

fn delete(item: Bool, m: Bool, v4: Int) -> Int {
case {
      let value = 0.1
      let z = value
      False
    }, "a" {
    True as whole, "" <> rest -> v4
    True, _ -> 5 + {
      {
        let item = 2
        let s = True
        3
      }
    }
    True, "a" -> v4 - 10
    _, _ -> fn(v5, v6) { v4 - 7 }(4, True)
  }
}

pub fn main() {
  let euler_value = case 1 {
    b -> fn(v7) { seed_value }("ab")
    inner -> {
      let inner = 10.0
      seed_value
    }
  }
  let seed_value = "x"
  echo {
    "data" <> {
      seed_value <> euler_value
    }
  } <> "a"
  echo {
    1 - 7
  } != 10
  echo fn(v8) { fn(v9, v10) { "b" <> euler_value }(0, "data") }(False)
}
