pub const limit_value: Int = 7
pub const euler_value: Bool = True

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(this_: Float) -> Bool {
True || {
    100 >= {
      4 % 1
    }
  }
}

fn f1(v0: List(Int), v1: String, new: Int) -> Float {
case {
      let v1 = v0
      v0
    } {
    [h, _, ..] if h % 2 == 0 -> {
      1.0
    } +. {
      100.0
    }
    [v0, ..rest] -> {
      fn(v2) { v2 }(0.0)
    } -. {
      100.0
    }
    _ -> case new |> spin(new) {
      1 -> 1.0
      0 | 2 -> {
        0.0
      } *. {
        2.0
      }
      inner -> fn(v3) { 1.0 }(True)
    }
  }
}

pub fn main() {
  echo {
    let new = {
      let x = {
        let constructor = []
        let n = 0.25
        "a"
      }
      let v = [2]
      {
        let v = "constructor"
        euler_value
      }
    }
    "bc"
  }
  echo 0.1
}
