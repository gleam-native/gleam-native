pub const pi_value: String = "res"
pub const golden_value: Float = 0.5

pub type V0 {
  Cv1(value: List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(v2: Float, length: V0, s: V0) -> List(Int) {
case spin(42, 5) {
    v3 -> case "a" <> "res", "a" {
      "bc", "x" <> rest if rest != "constructor" -> {
        let length = "b"
        let new = False
        [1]
      }
      "b" <> _ as whole, "abc" -> [4]
      v4, _ -> []
    }
    item -> case item - 0 {
      inner -> [0]
      b -> [4]
    }
  }
}

fn f1(v5: #(Float, String), arguments: V0) -> String {
{
    fn(v6, v7) { {
      let arguments = 4
      let x = arguments
      v7
    } }(10.0, "b")
  } <> {
    "ab" <> "ab"
  }
}

pub fn main() {
  let pi_value = "bc"
  echo [42]
  echo golden_value
  echo "b"
  echo case pi_value <> pi_value {
    "a" <> rest -> []
    "b" <> rest | "res" <> rest -> {
      let golden_value = 4 + 2
      let golden_value = 100.0
      [3, 0]
    }
    _ -> case {
        let rest = [5, 42]
        True
      }, Cv1([]) {
      True as whole, Cv1([6, ..rest]) if whole -> rest
      False, Cv1([5, 1, ..]) as whole -> []
      v8, v9 -> fn(v10) { [] }(True)
    }
  }
}
