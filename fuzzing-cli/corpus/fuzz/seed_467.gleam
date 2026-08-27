pub type Map {
  Cv0(value: String, inner: Float)
}

pub type V1 {
  Cv2(value: String, inner: List(Int))
  Ok(value: Int, inner: Float)
}

pub type Record {
  Cv3(List(Int), value: Int)
  Cv4
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v5: String) -> Int {
case "" <> v5 {
    b -> spin(fn(v6) { 1 }("abc"), fn(v7) { 2 }("data"))
    constructor -> fn(v8, v9) { v8 }(0, "x")
    item -> case Cv0("bc", 100.0) {
      Cv0(_, inner) -> 4
      _ -> {
        let length = 0.25
        let y = []
        10
      }
      Cv0(_, a) -> 0 * 2
    }
  }
}

fn class(default: Float) -> List(Int) {
[3]
}

fn f2(l: Int, self_: List(Int), x: #(List(Int), String)) -> List(Int) {
[]
}

pub fn main() {
  let n = [3, 100]
  let x = 0.0
  echo {
    {
      42 * 0
    } + {
      fn(v10, v11) { 10 }(5, 0.5)
    }
  } % 2
  echo case False && True {
    True -> case <<"a":utf8>>, 10 {
      <<_:big-unsigned-1, _:big-signed-8>> as whole, 2 -> 10.0
      <<3:8>>, _ -> fn(v12) { x }(42)
      _, _ -> {
        let x = x
        let this_ = False
        x
      }
    }
    arguments -> x
    constructor -> case 100 {
      item -> x
      constructor -> x *. x
    }
  }
  echo n
  echo ""
}
