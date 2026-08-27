pub const seed_value: Int = 3
pub const tag_value: Int = 5
pub const golden_value: Int = 4

pub type Symbol {
  Cv0(value: String, inner: Bool)
  Cv1(value: Bool, inner: Int)
  Ok
}

pub type Promise {
  Cv2(Int, value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(pair: List(Int), v3: Int) -> Float {
{
    3.14
  } /. {
    0.5
  }
}

fn arguments(self_: String) -> Float {
{
    {
      0.25
    } +. {
      0.25
    }
  } /. {
    2.0
  }
}

pub fn main() {
  let seed_value = 0.5
  echo case Cv2(2, "constructor") {
    z -> "abc"
    b -> case walk([], golden_value) {
      seed_value -> {
        let default = 4
        let n = "a"
        n
      }
      6 -> "" <> "res"
      inner -> "b"
    }
  }
  echo []
  echo case True, {
      let x = seed_value
      let x = 0.5
      True
    } {
    True, True as whole -> case {
        0.1
      } != {
        1.0
      } {
      constructor -> [0]
      _ | False -> []
      False -> []
    }
    True as whole, False -> [100, 0]
    _, v4 -> {
      let delete = False && v4
      let item = seed_value
      [3]
    }
  }
  echo "constructor"
}
