fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: List(Int), acc: String) -> Int {
{
    spin(constructor, constructor) % 4
  } * constructor
}

fn static(v1: Int, new: Int) -> Bool {
True
}

fn f2(self_: #(Bool, Float), v2: Int, v3: Int) -> String {
"bc"
}

pub fn main() {
  echo f2(#(True, 1.5), 3, {
    let this_ = 5
    this_
  }) <> {
    {
      let rest = 4
      "b"
    }
  }
  echo {
    let v = f2(#(False, 1.0), spin(100, 2), 0)
    let y = {
      0.5
    } <. {
      {
        let class = True
        1.0
      }
    }
    fn(v4, v5) { y }(3.14, "a")
  }
  echo fn(v6, v7) { 0.1 }(5, "bc")
}
