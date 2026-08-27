pub type Number {
  Cv0(value: String, inner: Float)
  Cv1
  Ok
}

pub type V2 {
  Cv3(List(Int))
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v5: Int, v6: V2) -> List(Int) {
[]
}

fn extends(rest: Float) -> Int {
spin({
    10 |> spin(4)
  } |> spin({
    let acc = True
    4
  }), {
    4 + 42
  } * 5)
}

fn yield(s: Int, z: Float, v7: Bool) -> Int {
7
}

pub fn main() {
  let class = 1
  let class = 42
  echo [3, 4]
}
