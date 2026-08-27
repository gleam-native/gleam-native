pub const euler_value: Bool = True

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
  Number
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(v3: String, v4: String, v5: List(Int)) -> List(Int) {
[1, 42]
}

fn default(constructor: Int, v6: Int) -> Bool {
constructor <= {
    {
      fn(v7) { v6 }("constructor")
    } - {
      constructor * constructor
    }
  }
}

fn class(class: String) -> Float {
{
    0.25
  } +. {
    {
      10.0
    } *. {
      {
        let class = [2, 3]
        100.0
      }
    }
  }
}

pub fn main() {
  echo 4
  echo class(fn(v8, v9) { v9 <> v9 }(False, "abc"))
  echo euler_value
}
