pub const pi_value: Int = 2

pub type V0 {
  None(value: String, inner: List(Int))
}

pub type Map {
  Cv1
  Cv2
  Cv3
}

pub type V4 {
  Error
}

fn f0(v5: Map) -> List(Int) {
case <<7:8, "res":utf8>>, 4 {
    <<_:utf8>>, 5 -> []
    _, 7 -> case fn(v6, v7) { v5 }("data", 0.25), 3 {
      Cv3, _ -> [4]
      _, 9 -> [1]
      _, _ -> fn(v8, v9) { [] }(100.0, False)
    }
    v10, _ -> [100]
  }
}

fn f1(x: String, v11: Bool, v12: Int) -> Int {
v12
}

fn default(z: List(Int)) -> Bool {
{
    case fn(v13) { True }(4) {
      False -> True
      item -> {
        let z = True
        False
      }
      True as whole -> 5 > 5
    }
  } || True
}

pub fn main() {
  let m = [4, 10]
  echo case 0 + pi_value {
    3 -> [42, 0]
    9 | 8 -> case Error {
      v14 -> f0(Cv1)
      Error -> [100]
    }
    7 -> Cv2 |> f0()
    _ -> m
  }
  echo "constructor"
  echo {
    let this_ = "bc"
    let m = case pi_value, pi_value {
      4 as whole, _ if whole == 2 -> pi_value - pi_value
      _, _ -> pi_value
      0, l -> f1(this_, False, l)
    }
    case {
        0.1
      } -. {
        0.1
      }, this_ {
      100.0, "ab" -> 1.5
      _, "a" -> 2.0
      _, v15 -> {
        let s = 7
        1.5
      }
    }
  }
}
