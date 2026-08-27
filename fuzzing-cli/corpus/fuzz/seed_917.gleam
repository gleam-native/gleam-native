pub const euler_value: String = ""
pub const pi_value: Int = 100
pub const limit_value: Int = 3

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Ok(value: Bool, inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v2: Bool, new: String) -> List(Int) {
case spin(7, 5) {
    item -> case Cv1([], 1) {
      b -> {
        let item = new
        let default = v2
        []
      }
      Cv1([8, ..rest], 2) -> [42]
    }
    1 -> case new, 1 {
      "abc" <> rest as whole, _ -> []
      "res" <> rest, _ -> []
      _, _ -> [0, 10]
    }
  }
}

fn extends(m: String, v3: V0) -> List(Int) {
case f0(True, "a"), v3 {
    [_, ..rest], m -> [0]
    [1, _, ..], Cv1([9], 9) -> []
    [0, constructor, ..], Ok(False, 1.5) -> [2, 4]
    _, _ -> []
  }
}

fn f2(pair: V0, v4: List(Int)) -> List(Int) {
{
    case {
        let s = False
        "data"
      } {
      constructor | "a" <> constructor -> fn(v5) { "res" }(0.5)
      b | "constructor" <> b -> "a"
    }
  } |> extends(pair)
}

pub fn main() {
  let euler_value = limit_value
  echo euler_value
  echo case pi_value |> spin({
      let euler_value = 1.0
      let limit_value = euler_value
      pi_value
    }), "x" {
    2, v6 if v6 == "ab" -> 1.0
    item, "data" -> 1.0
    _, v7 -> case v7 <> v7, fn(v8, v9) { v7 }(0.5, "constructor") {
      "x", "data" -> {
        let z = 10
        0.25
      }
      _, "bc" -> {
        0.1
      } +. {
        1.5
      }
      _, v10 -> 2.0
    }
  }
  echo 42
  echo "x" <> {
    {
      fn(v11, v12) { "" }(True, True)
    } <> "abc"
  }
}
