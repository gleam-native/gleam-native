pub type V0 {
  Cv1
  Cv2
  Cv3(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(prototype: Float, v4: Bool) -> Float {
fn(v5) { prototype +. {
    prototype /. {
      3.14
    }
  } }(True)
}

pub fn main() {
  let s = [100]
  echo []
  echo {
    {
      {
        1.0
      } +. {
        3.14
      }
    } +. {
      1.0
    }
  } >. {
    {
      {
        3.14
      } -. {
        10.0
      }
    } *. {
      3.14
    }
  }
  echo case 3.14 {
    _ | 10.0 -> [5]
    0.25 | 1.0 -> []
    2.0 | 0.25 -> s
  }
}
