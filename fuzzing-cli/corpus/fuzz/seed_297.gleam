pub const tag_value: Float = 0.5
pub const pi_value: Int = 1
pub const seed_value: String = "abc"

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(value: String, inner: List(Int))
}

pub type Map {
  Cv4(Int, List(Int))
}

fn f0(class: Float, value: Bool, v5: Int) -> List(Int) {
[]
}

fn f1(y: Int, z: Int) -> String {
{
    case Cv1 {
      Cv1 | Cv1 -> "b"
      a -> ""
    }
  } <> {
    fn(v6, v7) { "x" }("constructor", 2)
  }
}

pub fn main() {
  echo pi_value
}
