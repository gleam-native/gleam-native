pub const seed_value: String = "data"
pub const limit_value: Float = 3.14
pub const euler_value: String = "abc"

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

fn f0(constructor: V0) -> List(Int) {
[4, 5]
}

pub fn main() {
  echo f0(Cv1([5, 7], 1))
}
