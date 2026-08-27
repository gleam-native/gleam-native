pub const limit_value: Bool = True
pub const seed_value: Int = 100

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(delete: Object) -> String {
"a" <> {
    case 0 % 4 {
      v0 -> "ab"
      2 -> "constructor"
      7 -> "bc"
    }
  }
}

fn f1(prototype: Object) -> Bool {
True
}

pub fn main() {
  echo limit_value
  echo fn(v1) { {
    0.5
  } -. {
    {
      let rest = v1
      let default = [2]
      0.25
    }
  } }("ab")
  echo seed_value
}
