pub const tag_value: Bool = True
pub const seed_value: String = ""
pub const euler_value: Float = 100.0

pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Record(value: Float, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(length: String) -> Bool {
{
    let length = case 0 - 4, <<2:16, "abc":utf8, "":utf8>> {
      7, <<item:8, _:utf8>> -> [100]
      _, _ -> {
        let length = 0
        []
      }
    }
    let default = {
      {
        0.1
      } /. {
        2.0
      }
    } +. {
      0.1
    }
    {
      let v = fn(v3, v4) { 1.5 }(0, 7)
      let pair = fn(v5, v6) { v5 }(3, 10.0)
      {
        0.1
      } <=. {
        1.5
      }
    }
  }
}

fn static(z: Int) -> Bool {
walk(fn(v7, v8) { [] }("ab", 1.0), 1 - z) <= {
    3 * 7
  }
}

pub fn main() {
  let prototype = case {
      let self_ = euler_value
      Cv2
    } {
    _ -> []
    Cv2 -> [10]
  }
  echo euler_value
}
