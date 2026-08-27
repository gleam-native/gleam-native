pub const limit_value: Float = 2.0
pub const seed_value: Int = 7

pub type V0 {
  Cv1
  Cv2
  Cv3(Float)
}

pub type Promise {
  Cv4(value: Float, inner: Int)
  Cv5(value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(s: String, constructor: #(Bool, List(Int)), v6: String) -> Bool {
True
}

fn extends(y: #(Int, List(Int)), m: List(Int), v7: Bool) -> List(Int) {
fn(v8, v9) { fn(v10, v11) { m }("b", "x") }(1.5, "data")
}

fn f2(delete: Bool, v12: V0, v13: #(Bool, Float)) -> Int {
case <<2:8, "":utf8, "abc":utf8>> {
    <<7:8>> -> 2
    <<_:utf8, 10:4, "res":utf8>> -> 5 - {
      42 + 0
    }
    _ -> 0
  }
}

pub fn main() {
  let pair = limit_value
  echo fn(v14, v15) { case {
      let rest = False
      let v = rest
      "res"
    } {
    _ -> extends(#(0, [7, 1]), [10, 5], False)
    "a" | "data" -> [4]
  } }(False, 1)
  echo "x"
}
