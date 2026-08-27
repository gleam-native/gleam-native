pub const limit_value: Float = 2.0
pub const euler_value: Float = 0.1

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Cv3(value: Bool, inner: List(Int))
}

fn f0(v4: Float) -> Bool {
{
    False || {
      False || True
    }
  } || {
    case 3, True {
      _, _ -> fn(v5, v6) { v6 }(3.14, True)
      v4, True as whole -> True || True
    }
  }
}

pub fn main() {
  let l = {
    let rest = 7 * 2
    let acc = rest + 1
    rest - rest
  }
  let m = case "abc" == "res" {
    True -> fn(v7, v8) { [] }(False, False)
    _ -> [5, 1]
  }
  echo m
  echo {
    {
      let m = limit_value
      2.0
    }
  } /. {
    2.0
  }
  echo m
}
