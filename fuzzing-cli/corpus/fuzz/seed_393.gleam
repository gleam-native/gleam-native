pub const tag_value: Bool = False
pub const euler_value: String = "b"
pub const seed_value: Int = 7

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(n: String) -> List(Int) {
case #(100.0, [100]) {
    #(3.14, [4] as whole) -> case "constructor", [1, 100] {
      v0, [3, 6, ..] if v0 == "a" || v0 == "res" -> whole
      "bc" <> rest, [3, ..tail] -> [4, 0]
      _, v1 -> whole
    }
    item -> [10, 1]
  }
}

fn f1(z: Int, y: Int, v2: Int) -> String {
"b"
}

fn delete(v3: Bool) -> String {
case fn(v4) { True }(0.0) {
    a -> "b"
    True -> "a"
    False -> "a"
  }
}

pub fn main() {
  echo fn(v5, v6) { {
    0.0
  } +. {
    fn(v7, v8) { 100.0 }(False, 1.0)
  } }(False, 100)
  echo case 42 == 1 {
    item -> fn(v9, v10) { {
      0.1
    } -. {
      2.0
    } }("", True)
    constructor -> {
      {
        0.5
      } -. {
        1.0
      }
    } +. {
      {
        let item = 42
        2.0
      }
    }
  }
  echo euler_value
}
