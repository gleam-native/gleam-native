pub const euler_value: Bool = False
pub const limit_value: String = "a"

pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type Number {
  Error
  Record(value: Bool, inner: List(Int))
  Cv3(Bool, value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(new: Float) -> Int {
3
}

fn f1(this_: V0, v4: Int, v5: V0) -> Bool {
case Cv3(False, "x") {
    Record(True, [a]) -> False
    Cv3(False, "constructor" <> rest) if rest != "x" -> {
      {
        let v4 = 3.14
        let this_ = 10
        0.5
      }
    } == {
      {
        1.5
      } -. {
        100.0
      }
    }
    Record(True, [1]) as whole -> case v4 - 3, #("a", 2.0) {
      7, #("abc", v5) -> {
        let v = True
        let delete = False
        v
      }
      0, #("x", v6) -> True
      6, #("bc" as whole, 2.0) -> False
      v7, _ -> False
    }
    _ -> True
  }
}

pub fn main() {
  let limit_value = fn(v8, v9) { {
    let s = v9
    limit_value
  } }(0.25, 4)
  let self_ = case {
      3.14
    } |> f0() {
    8 -> euler_value
    2 -> fn(v10) { False }(7)
    constructor -> False
  }
  echo True
  echo [5, 4]
}
