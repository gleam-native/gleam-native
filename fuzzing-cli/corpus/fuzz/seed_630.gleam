pub type V0 {
  Some(value: String, inner: String)
  Cv1
  None(value: List(Int), inner: String)
}

pub type V2 {
  Cv3(Bool, Float)
  Cv4(value: Float)
  Cv5(Int)
}

pub type V6 {
  Cv7(Float, Float)
  Number
  Record(List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn static(constructor: List(Int), this_: Bool, v8: V2) -> Bool {
this_
}

pub fn main() {
  echo "data"
  echo {
    {
      let length = "abc" <> "data"
      let constructor = {
        1.5
      } +. {
        2.0
      }
      []
    }
  } |> static(3 > 100, {
    let l = "abc"
    let arguments = l
    Cv4(0.5)
  })
  echo 100
  echo ""
}
