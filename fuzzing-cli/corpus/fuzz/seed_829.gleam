fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, v: Bool) -> Bool {
False
}

fn f1(this_: String, x: Int) -> Int {
x
}

pub fn main() {
  echo 0.25
  echo 10.0
  echo {
    {
      2.0
    } /. {
      3.14
    }
  } +. {
    {
      fn(v1) { 2.0 }("constructor")
    } -. {
      2.0
    }
  }
  echo {
    fn(v2, v3) { 10.0 }("a", 4)
  } -. {
    fn(v4, v5) { {
      3.14
    } -. {
      10.0
    } }("abc", 5)
  }
}
