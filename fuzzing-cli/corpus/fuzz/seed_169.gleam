pub const seed_value: String = ""

pub type Number {
  Cv0(value: String, inner: List(Int))
}

pub type Map {
  Error(List(Int), String)
  Cv1(Int, value: Int)
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn arguments(v3: #(Float, Bool), constructor: Int, self_: Int) -> Int {
constructor
}

fn new(y: Bool) -> List(Int) {
case y && y, 5 {
    True, 5 -> case <<"":utf8, "x":utf8>>, 10 {
      <<length:4, _:big-unsigned-16>> as whole, 0 if length <= 7 || length <= 9 -> [5, 10]
      <<7:8>>, y if y <= 6 || y % 2 == 0 -> [2]
      _, 4 -> [3, 2]
      _, _ -> []
    }
    True, 3 -> []
    _, 7 -> [1]
    _, _ -> fn(v4, v5) { [100] }(100, 0.1)
  }
}

pub fn main() {
  let seed_value = 3 + {
    {
      let prototype = seed_value
      7
    }
  }
  echo True |> new()
  echo {
    case 2 {
      item -> 0.1
      5 -> {
        3.14
      } -. {
        0.1
      }
      inner -> {
        let s = "x"
        0.1
      }
    }
  } +. {
    {
      let n = True
      0.0
    }
  }
}
