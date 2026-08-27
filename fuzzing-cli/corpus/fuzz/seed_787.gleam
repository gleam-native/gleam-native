pub const limit_value: Float = 0.5
pub const pi_value: String = ""
pub const tag_value: Float = 2.0

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(pair: Int) -> List(Int) {
[100]
}

fn f1(v0: Float) -> List(Int) {
fn(v1, v2) { fn(v3, v4) { [0, 4] }(10.0, 7) }("abc", "b")
}

fn f2(s: Bool, n: Int, new: List(Int)) -> Float {
{
    {
      let z = 1.5
      let y = z
      y
    }
  } +. {
    fn(v5) { {
      let delete = "a"
      let length = 10.0
      0.25
    } }("abc")
  }
}

pub fn main() {
  let tag_value = pi_value
  echo {
    case f1(0.0) {
      [] -> 1.5
      [5] -> 0.0
      [] -> limit_value *. limit_value
      _ -> True |> f2(3, [2, 1])
    }
  } *. {
    {
      {
        let self_ = True
        1.5
      }
    } *. {
      {
        let s = limit_value
        3.14
      }
    }
  }
  echo pi_value
  echo case walk([], 42) {
    0 | 1 -> case "data", fn(v6, v7) { v6 }(10, "bc") {
      "res", 8 -> True
      _, pair -> False
    }
    constructor -> {
      constructor + constructor
    } <= walk([0, 100], constructor)
  }
  echo f2(True && {
    fn(v8, v9) { False }("a", "")
  }, case constructor(1), [0] {
    [4], [a, x, ..] as whole -> walk([7], x)
    [3, 1, ..], [8, a, ..] -> a % 3
    _, v10 -> 4 % 1
  }, case 100 < 7 {
    inner -> [2]
    v11 -> [42]
  })
}
