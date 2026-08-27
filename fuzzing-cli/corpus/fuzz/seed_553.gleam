pub const euler_value: Bool = True
pub const limit_value: String = "a"

pub type V0 {
  Error(value: String, inner: Float)
}

pub type Map {
  Record(List(Int), Int)
  Cv1(Bool)
  Cv2(String)
}

pub type V3 {
  Cv4(value: Bool, inner: String)
  Cv5(value: Float, inner: Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v6: Float, z: Int, prototype: V0) -> String {
"b"
}

fn static(v7: String, v8: #(String, Bool), arguments: Bool) -> Float {
1.0
}

fn f2(acc: Bool, default: Int) -> List(Int) {
fn(v9) { case fn(v10) { [] }(2) {
    [] as whole -> whole
    [] -> {
      let v9 = [100]
      v9
    }
    [0, ..rest] -> []
    _ -> []
  } }("")
}

pub fn main() {
  let length = 3.14
  let n = [10]
  echo case length -. {
      100.0
    } {
    _ -> fn(v11, v12) { f2(v11, 5) }(True, 5)
    1.0 | 10.0 -> [7, 3]
  }
  echo {
    let item = 4
    let class = n
    {
      let pair = {
        3.14
      } +. length
      10
    }
  }
  echo limit_value
}
