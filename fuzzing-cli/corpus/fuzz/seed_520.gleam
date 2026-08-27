pub const limit_value: String = "b"
pub const euler_value: String = "abc"
pub const golden_value: Bool = True

pub type V0 {
  Cv1
  Cv2
  Cv3(Bool, value: Float)
}

pub type V4 {
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v6: #(String, Int), v7: String) -> Float {
0.5
}

fn f1(new: Bool, acc: V4, n: V0) -> Bool {
{
    {
      let m = False
      let n = "ab" <> "b"
      n <> n
    }
  } != {
    "constructor" <> "constructor"
  }
}

fn f2(v8: Int, value: V0) -> Float {
#("constructor", 100) |> f0(fn(v9, v10) { "res" }(4, "res"))
}

pub fn main() {
  echo walk(case fn(v11) { golden_value }(0.5) {
    b -> [100, 42]
    False -> [5]
  }, 2)
  echo {
    {
      10.0
    } >. {
      {
        3.14
      } -. {
        0.25
      }
    }
  } || {
    f1(golden_value, Cv5, Cv3(True, 3.14)) |> f1(Cv5, Cv3(False, 1.5))
  }
}
