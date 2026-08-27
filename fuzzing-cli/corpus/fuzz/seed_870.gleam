pub const limit_value: Bool = False
pub const euler_value: Bool = False
pub const pi_value: Int = 5

pub type Map {
  Cv0(value: String, inner: Bool)
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Float)
}

pub type V3 {
  Cv4(String)
  Cv5
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn default(v6: String, item: List(Int)) -> String {
v6
}

fn f1(z: #(Bool, Int), acc: String, v7: #(Float, Bool)) -> Float {
{
    let constructor = fn(v8) { 1.5 }(True)
    constructor +. {
      3.14
    }
  }
}

fn f2(this_: Int, class: String) -> List(Int) {
fn(v9, v10) { [] }(False, 100.0)
}

pub fn main() {
  echo case {
      let s = 0.1
      let euler_value = "res"
      [7]
    } {
    [8, ..rest] -> [2]
    [0] -> pi_value |> f2("a" <> "x")
    _ -> fn(v11) { v11 |> f2(fn(v12) { "abc" }(4)) }(10)
  }
  echo []
  echo {
    !True
  } || True
}
