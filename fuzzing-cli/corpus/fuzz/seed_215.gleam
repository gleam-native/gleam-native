pub type V0 {
  Record(value: String, inner: List(Int))
  None(value: List(Int))
  Some(value: String)
}

pub type V1 {
  Cv2(String, Int)
  Cv3(value: Bool, inner: Int)
  Cv4
}

pub type V5 {
  Cv6(Int, Bool)
  Cv7(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(length: Int, arguments: Bool) -> Int {
{
    case length {
      6 | 0 -> {
        let constructor = 100.0
        length
      }
      5 -> length
      b -> length
    }
  } - length
}

fn new(self_: Bool, class: Int) -> Bool {
case {
      let this_ = class
      Some("x")
    } {
    Record("x", [2, 8, ..]) -> {
      let self_ = [7]
      True
    }
    inner -> {
      {
        2.0
      } *. {
        1.0
      }
    } >=. {
      {
        10.0
      } +. {
        2.0
      }
    }
  }
}

pub fn main() {
  echo {
    {
      10.0
    } == {
      0.0
    }
  } || {
    spin(5, 4) != {
      fn(v8) { 0 }(True)
    }
  }
}
