pub const golden_value: Int = 0
pub const seed_value: Float = 1.0

pub type V0 {
  Ok(value: String, inner: List(Int))
  Cv1(value: Int)
}

pub type V2 {
  Error(Bool, Bool)
  None
  Cv3(Int)
}

pub type Map {
  Cv4(value: String, inner: List(Int))
  Cv5(Int, value: List(Int))
}

fn constructor(self_: Int, s: Float, n: String) -> Bool {
{
    let n = fn(v6, v7) { s +. {
      1.0
    } }(False, True)
    let self_ = True
    False
  }
}

pub fn main() {
  echo [42]
  echo True
}
