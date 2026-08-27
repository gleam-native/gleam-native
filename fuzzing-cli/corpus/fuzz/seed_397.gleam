pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type V3 {
  Error
  Cv4
  Cv5(value: Bool)
}

pub type V6 {
  Cv7
}

fn f0(n: String, m: Int) -> String {
"b"
}

fn class(x: Int) -> List(Int) {
[]
}

fn f2(pair: List(Int), v8: List(Int), l: Bool) -> Float {
{
    case class(4), f0("bc", 5) {
      [] as whole, "data" -> 1.5
      [2, pair, ..], "bc" -> {
        let rest = 100.0
        0.5
      }
      v9, _ -> 0.5
    }
  } /. {
    10.0
  }
}

pub fn main() {
  let x = False
  let l = class(42)
  echo {
    {
      4 % 2
    } - {
      0 * 7
    }
  } - {
    5 + 1
  }
  echo case Cv2 {
    Cv2 -> x
    Cv1([]) | Cv1(_) -> "res" != {
      "data" <> "data"
    }
    Cv2 -> {
      1.0
    } <=. {
      10.0
    }
    v10 -> x
  }
  echo 0.5
}
