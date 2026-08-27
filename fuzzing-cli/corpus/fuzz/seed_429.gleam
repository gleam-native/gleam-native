pub const limit_value: Float = 1.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(String, value: Int)
  Cv3
}

pub type V4 {
  Cv5
  Cv6
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(this_: Bool, acc: Int) -> Bool {
False
}

fn f1(arguments: String, class: V0, v7: #(List(Int), String)) -> Int {
{
    case #(True, False), fn(v8, v9) { False }("b", 1.0) {
      #(False, _) as whole, False -> 100 + 2
      #(False, True as whole), _ -> spin(42, 7)
      _, v10 -> fn(v11, v12) { v12 }(4, 0)
    }
  } % 7
}

pub fn main() {
  let limit_value = limit_value
  echo {
    let acc = 3
    fn(v13) { "a" <> "b" }(4)
  }
  echo [10]
  echo {
    case Cv3 {
      _ -> 100.0
      Cv1([8], 1) | Cv1(_, _) -> 10.0
      Cv3 -> limit_value -. {
        0.1
      }
    }
  } *. limit_value
  echo 3.14
}
