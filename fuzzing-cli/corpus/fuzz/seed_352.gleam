pub const seed_value: Float = 2.0
pub const euler_value: Float = 1.5
pub const tag_value: Int = 3

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, l: Bool, v0: String) -> Bool {
True
}

fn f1(v1: List(Int)) -> String {
{
    {
      let this_ = fn(v2) { False }("ab")
      let this_ = {
        let class = True
        "ab"
      }
      fn(v3) { this_ }(3)
    }
  } <> "b"
}

fn constructor(new: Int) -> String {
"constructor"
}

pub fn main() {
  let n = 42
  let s = {
    let x = 1.0
    "ab" != "bc"
  }
  echo "ab"
  echo [7]
  echo True
  echo {
    {
      let z = 4 > n
      let n = {
        let s = 1
        2.0
      }
      [1, 4]
    }
  } |> f1()
}
