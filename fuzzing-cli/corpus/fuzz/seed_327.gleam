pub const limit_value: Int = 2
pub const golden_value: Int = 7
pub const pi_value: Float = 0.25

pub type V0 {
  Cv1
  Cv2
  Cv3
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v4: Int) -> Bool {
case "constructor" <> "constructor" {
    constructor -> True || True
    "res" <> inner | "x" <> inner -> {
      2 >= 42
    } || {
      fn(v5) { False }("x")
    }
    "b" -> False
  }
}

pub fn main() {
  let s = 2.0
  echo 42
  echo {
    fn(v6) { {
      let delete = 0.0
      let constructor = "data"
      ""
    } }(0)
  } <> {
    {
      fn(v7) { "b" }(0.0)
    } <> {
      fn(v8) { "a" }(True)
    }
  }
}
