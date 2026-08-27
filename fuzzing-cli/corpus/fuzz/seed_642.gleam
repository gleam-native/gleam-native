pub const euler_value: String = "x"

pub type Map {
  Cv0(value: String, inner: String)
}

pub type Symbol {
  Ok(value: Bool, inner: List(Int))
  Cv1
  Cv2(Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(pair: Map, v: List(Int), prototype: List(Int)) -> List(Int) {
v
}

pub fn main() {
  let length = {
    fn(v3) { 1 }(0.1)
  } * {
    10 - 7
  }
  let euler_value = {
    0.1
  } *. {
    {
      let n = ""
      let value = 0.5
      value
    }
  }
  echo 0.5
  echo {
    let default = case Cv0("res", "abc") {
      Cv0(constructor, _) if constructor == "a" -> True
      constructor -> fn(v4, v5) { True }("data", 42)
      Cv0(_, "constructor" <> rest) -> "b" != rest
    }
    fn(v6) { [] }(3.14)
  }
}
