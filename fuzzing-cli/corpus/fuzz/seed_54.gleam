pub const euler_value: Bool = False
pub const golden_value: Float = 1.0
pub const seed_value: Float = 0.5

pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v2: String) -> List(Int) {
case v2 <> v2, 100 {
    "x", _ -> case v2, fn(v3) { "b" }(10) {
      "constructor" as whole, _ if whole != "ab" || whole != "abc" -> {
        let v2 = 10
        [42]
      }
      "res" as whole, "b" as it -> [1]
      v4, _ -> [2, 100]
    }
    "b" <> rest, 8 -> fn(v5, v6) { [3, 3] }(True, "a")
    v7, _ -> case "a", {
        let v2 = 42
        let self_ = v2
        "x"
      } {
      "ab", _ -> {
        let v7 = []
        v7
      }
      "a" <> rest as whole, _ -> []
      _, v8 -> fn(v9, v10) { [] }(True, 5)
    }
  }
}

pub fn main() {
  echo 0
  echo 10
  echo case {
      let default = 2.0
      Cv1([])
    }, 1 {
    Cv1([_, ..rest]), this_ if this_ == 7 -> [5]
    Cv1([3, _, ..]), 5 -> [3]
    v11, _ -> []
  }
}
