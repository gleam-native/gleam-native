pub const euler_value: String = "x"

pub type V0 {
  None(value: String, inner: Int)
}

pub type Record {
  Cv1(List(Int))
  Cv2
  Cv3
}

pub type V4 {
  Cv5
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v6: List(Int), s: Int, v7: V0) -> Float {
case "b" {
    constructor -> 0.1
    "ab" <> rest as whole -> {
      fn(v8, v9) { 1.5 }(5, 100)
    } +. {
      1.5
    }
  }
}

fn f1(v10: Int, delete: List(Int), z: String) -> Bool {
z == {
    case v10, "res" {
      4 as whole, _ -> {
        let pair = 3.14
        z
      }
      8, "abc" <> rest as whole -> fn(v11, v12) { "" }("ab", 1)
      _, v13 -> v13
    }
  }
}

fn f2(v14: V4, s: String) -> List(Int) {
[]
}

pub fn main() {
  echo {
    let s = case Cv5 {
      Cv5 -> [1]
      b -> [2]
    }
    {
      0 * 0
    } + 10
  }
}
