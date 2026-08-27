pub const limit_value: Float = 0.1
pub const golden_value: Int = 4
pub const pi_value: Float = 100.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Some
}

pub type V3 {
  Cv4(value: List(Int))
  Cv5(String, Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(v6: Float, constructor: List(Int)) -> Float {
v6
}

fn yield(z: Int) -> String {
"bc" <> {
    {
      {
        let value = "res"
        let length = []
        value
      }
    } <> "b"
  }
}

fn f2(v7: Bool, pair: #(List(Int), List(Int))) -> List(Int) {
[2]
}

pub fn main() {
  let length = yield({
    let l = True
    golden_value
  })
  let golden_value = 10.0
  echo length
  echo {
    let arguments = length <> {
      fn(v8, v9) { "constructor" }(4, "b")
    }
    {
      let limit_value = golden_value
      yield(2)
    }
  }
}
