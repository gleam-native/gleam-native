pub const euler_value: Int = 2

pub type Object {
  Cv0(value: String, inner: String)
  Record(value: Float, inner: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(s: #(Int, Float), v1: Int, v2: Float) -> Float {
v2
}

fn f1(acc: String, v3: #(Int, String), value: Int) -> Bool {
{
    "a" <> "a"
  } != {
    {
      let l = [42]
      acc <> "ab"
    }
  }
}

pub fn main() {
  let self_ = case False {
    _ -> [2]
    inner -> []
    b -> [4]
  }
  let euler_value = walk(self_, euler_value + euler_value)
  echo []
  echo euler_value * {
    case "constructor" |> f1(#(10, "a"), 3 % 7) {
      True | True -> 1
      _ | False -> walk([1], 7)
      True -> euler_value
    }
  }
  echo euler_value + walk({
    let z = ""
    self_
  }, euler_value + 10)
  echo True
}
