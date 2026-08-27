pub const limit_value: Bool = False
pub const euler_value: Bool = True

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3
}

pub type V4 {
  Cv5(Int)
  Cv6(value: Bool)
  Cv7(value: List(Int), inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v8: Bool, v9: Float) -> Float {
case 4 - 1, [] {
    6, [9] -> case v9, {
        let z = v8
        let class = v9
        Cv1
      } {
      1.5, Cv1 -> v9
      2.0, Cv1 -> 0.5
      100.0, Cv1 -> v9
      _, _ -> 2.0
    }
    v10, [7, b, ..] -> 3.14
    8, [2] -> {
      let v8 = 0
      let acc = {
        let delete = 10.0
        "x"
      }
      v9
    }
    _, v11 -> case #("data", 42), Cv3 {
      #("x", 4) as whole, _ -> 0.25
      #("b" as whole, 1), Cv3 if whole == "a" -> v9
      #("b" <> _, 9), Cv3 -> v9 +. v9
      v12, v13 -> {
        let n = True
        let v9 = "x"
        2.0
      }
    }
  }
}

fn arguments(v14: Int, v15: Int, acc: Int) -> Float {
1.0
}

pub fn main() {
  let length = [4, 0]
  let length = 0.25
  echo 1.0
}
