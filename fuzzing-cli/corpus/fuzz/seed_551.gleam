pub const seed_value: Int = 3
pub const euler_value: Float = 0.5

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: String)
  Error(Int)
}

pub type Symbol {
  Some
  Record(Float, Int)
}

fn f0(value: Int, y: Symbol) -> List(Int) {
case [7, 0], fn(v3) { value }(7) {
    [h, 4, ..], 5 -> {
      let value = [1, 3]
      fn(v4, v5) { [4, 4] }("b", True)
    }
    [y, ..rest] as whole, v6 -> case "data", {
        0.5
      } >. {
        10.0
      } {
      _, False as whole if !whole -> rest
      _, True -> {
        let s = rest
        [2]
      }
      v7, v8 -> whole
    }
    _, v9 -> case Cv1([], 0), <<"x":utf8, "res":utf8>> {
      v10, <<0:16, 0:4, this_:1>> if this_ > 4 -> [0, 0]
      Cv2("res" <> rest), <<_:utf8, 10:8, 42:16>> -> []
      Error(4), _ -> fn(v11) { [0, 3] }(4)
      v12, v13 -> [3]
    }
  }
}

fn f1(acc: String) -> Float {
{
    10.0
  } /. {
    0.5
  }
}

pub fn main() {
  echo {
    {
      "b" <> "res"
    } <> "data"
  } <> ""
}
