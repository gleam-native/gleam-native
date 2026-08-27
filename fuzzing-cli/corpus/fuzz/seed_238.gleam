pub const tag_value: Int = 0

pub type Number {
  Cv0(value: String, inner: String)
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(value: Float, inner: Bool)
  Cv4(Int, List(Int))
  Cv5(value: Float, inner: Int)
}

pub type Map {
  Cv6(value: Bool, inner: Float)
  Record(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v7: Bool, v8: Int, delete: Float) -> Int {
v8
}

fn f1(v9: Float, v: Map, y: Number) -> Int {
5 * 0
}

fn f2(acc: List(Int), v10: Float, y: Float) -> Int {
5
}

pub fn main() {
  echo {
    2.0
  } <. {
    case #(True, []), {
        let value = [10]
        let value = value
        4
      } {
      #(True, []), _ -> 1.0
      #(_, []), 2 -> {
        0.5
      } -. {
        0.25
      }
      #(False, [constructor, ..rest]), tag_value -> 3.14
      v11, _ -> 0.0
    }
  }
  echo {
    10.0
  } -. {
    {
      {
        0.1
      } +. {
        0.1
      }
    } -. {
      {
        10.0
      } -. {
        3.14
      }
    }
  }
}
