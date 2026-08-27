pub const tag_value: Bool = True
pub const euler_value: String = ""
pub const golden_value: Bool = True

pub type V0 {
  Some(value: String, inner: String)
  Cv1(Bool, value: Float)
  Ok(Int)
}

pub type V2 {
  Number(List(Int))
}

pub type V3 {
  Cv4(value: Float)
  Cv5(value: List(Int), inner: Bool)
  Error(Bool)
}

fn f0(l: Int) -> Bool {
False
}

fn f1(this_: Int) -> Bool {
fn(v6, v7) { v6 }(False, 0.5)
}

pub fn main() {
  let class = case <<"":utf8, "x":utf8, 42:8>> {
    <<_:little-unsigned-1, _:8, "res":utf8>> as whole -> 3.14
    <<_:utf8>> -> {
      0.5
    } -. {
      10.0
    }
    _ -> fn(v8, v9) { 0.0 }(4, 3.14)
  }
  echo 3.14
  echo []
  echo 5
}
