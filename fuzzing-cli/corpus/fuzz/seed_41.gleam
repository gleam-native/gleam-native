pub const pi_value: Bool = True

pub type V0 {
  Some(value: String, inner: Float)
}

pub type Record {
  None(value: List(Int))
  Cv1
}

pub type V2 {
  Cv3(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(v4: Bool, value: Float) -> Float {
case <<42:4, 2:8, 2:4>>, value {
    <<3:8>>, 0.1 -> fn(v5) { value *. value }(2)
    _, 0.25 -> {
      let n = "" <> "data"
      {
        10.0
      } *. value
    }
    v6, _ -> 100.0
  }
}

fn constructor(v7: #(List(Int), Int)) -> Bool {
True
}

pub fn main() {
  let new = {
    let m = "x"
    let default = 1.5
    {
      let pair = 42
      pair
    }
  }
  echo {
    fn(v8) { 5 }(42)
  } % 2
  echo [100, 1] |> walk(new)
}
