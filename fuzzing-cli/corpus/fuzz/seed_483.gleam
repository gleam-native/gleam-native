pub const euler_value: String = "constructor"
pub const seed_value: Bool = True

pub type Map {
  Cv0(value: String, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(prototype: Float, length: Int, v1: Int) -> String {
"constructor"
}

fn yield(v2: Int, v3: Float, v4: #(Float, Float)) -> String {
f0(100.0, v2 - 10, v2) <> {
    fn(v5, v6) { v5 }("b", "constructor")
  }
}

pub fn main() {
  echo case euler_value, 1 {
    _, 0 -> seed_value
    "ab", 7 -> False
    v7, v8 -> {
      {
        1.5
      } *. {
        1.0
      }
    } <. {
      {
        10.0
      } -. {
        1.5
      }
    }
  }
}
