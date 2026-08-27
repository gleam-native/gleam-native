pub type Number {
  Cv0(value: String, inner: Float)
  Cv1(value: List(Int))
}

pub type Map {
  Cv2
  Cv3
  Record
}

pub type V4 {
  Cv5(Bool, Int)
  Number(value: Int, inner: Bool)
  Cv6
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v7: #(Float, Float), v8: V4, rest: List(Int)) -> List(Int) {
{
    let z = {
      let arguments = False
      True
    }
    [100]
  }
}

fn export(v9: #(Int, Float), v10: Int) -> Bool {
True
}

fn arguments(v11: Bool, l: Int, x: Int) -> List(Int) {
case {
      let z = 1.0
      let x = "constructor"
      5
    }, {
      let y = 1.0
      let class = [3, 0]
      l
    } {
    7, 3 as whole -> #(1.0, 0.5) |> f0(Number(2, False), fn(v12) { [10] }(10.0))
    2, _ -> []
    v13, _ -> #(0.5, 0.5) |> f0(Number(7, True), [4])
  }
}

pub fn main() {
  let class = 1.0
  let class = "data"
  echo case True || True, fn(v14) { 0 }(100.0) {
    _, y -> "data"
    False, v15 -> {
      let arguments = 100
      class <> class
    }
  }
  echo [10]
}
