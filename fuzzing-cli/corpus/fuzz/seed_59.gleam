pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Cv3
}

pub type V4 {
  Cv5(String, Bool)
  Cv6(value: String)
}

pub type V7 {
  Cv8
  None
}

fn f0(constructor: Int) -> Int {
constructor + {
    {
      4 - constructor
    } + {
      {
        let rest = False
        1
      }
    }
  }
}

fn f1(default: Int) -> Bool {
f0(default - 10) == {
    default - 7
  }
}

fn f2(v9: #(String, Int), pair: Int, v: List(Int)) -> Int {
{
    0 + {
      {
        let item = True
        let v9 = "bc"
        pair
      }
    }
  } + {
    {
      pair + pair
    } + {
      5 + pair
    }
  }
}

pub fn main() {
  let this_ = 1
  echo {
    case 1.0 {
      b -> "bc"
      item -> {
        let item = "bc"
        let m = this_
        item
      }
      _ -> "res"
    }
  } == "a"
}
