pub const pi_value: Float = 3.14

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(y: Int, class: Float, s: Bool) -> String {
"bc" <> "bc"
}

pub fn main() {
  echo 5
  echo case fn(v0) { True }(True) {
    True -> case 0 + 100, Record {
      _, Record -> [7, 2]
      7, _ -> fn(v1, v2) { [] }(True, 0.25)
      _, _ -> [4]
    }
    inner -> case pi_value {
      constructor -> {
        let pi_value = "abc"
        [10]
      }
      constructor -> [4]
      inner -> []
    }
    new -> []
  }
  echo fn(v3, v4) { {
    {
      let default = False
      let v3 = 100
      pi_value
    }
  } -. {
    fn(v5, v6) { pi_value }(100, 2)
  } }("b", "ab")
}
