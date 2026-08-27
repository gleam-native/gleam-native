pub const euler_value: Int = 1
pub const tag_value: Bool = False

pub type Promise {
  Cv0(value: String, inner: Float)
}

fn f0(pair: Float, item: Int) -> Bool {
{
    case pair +. {
        0.0
      } {
      b -> b
      item -> item
    }
  } == pair
}

pub fn main() {
  let euler_value = case fn(v1, v2) { "a" }("bc", 100) {
    "abc" | "b" <> _ -> True
    v3 -> True
    "constructor" <> _ -> f0(0.0, euler_value)
  }
  echo 0.5
  echo {
    1 % 7
  } - 3
  echo 42 * {
    7 % 7
  }
}
