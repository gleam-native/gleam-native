pub const euler_value: Int = 100
pub const limit_value: String = ""

pub type V0 {
  Cv1(value: List(Int))
  Cv2(List(Int))
  Cv3(value: String)
}

pub type V4 {
  Cv5
}

pub type Promise {
  Cv6(Int, String)
  Error(Float)
}

fn new(length: Bool, x: Int) -> Float {
case 42 {
    b -> case Cv1([]) {
      Cv3(_) -> 10.0
      Cv3(_) -> {
        1.5
      } -. {
        1.5
      }
      Cv3("a") | Cv2(_) -> 0.0
      v7 -> 1.5
    }
    class -> {
      1.0
    } *. {
      3.14
    }
    2 -> fn(v8, v9) { {
      let n = 0.0
      let acc = []
      n
    } }("constructor", 0.5)
  }
}

fn default(v10: Int, delete: Int) -> Bool {
{
    case {
        3.14
      } == {
        2.0
      }, <<0:8, 7:16, 2:4>> {
      _, <<_:little-signed-16, l:4>> -> v10
      True, _ -> 42 % 5
      v11, _ -> {
        let v10 = 1.0
        let delete = 10.0
        7
      }
    }
  } >= {
    case "abc" {
      "b" <> rest if rest != "a" -> delete
      _ -> 1
      "abc" -> v10 + 42
    }
  }
}

fn export(v12: Bool) -> Bool {
v12
}

pub fn main() {
  let limit_value = "data"
  let limit_value = case limit_value <> "ab", {
      1.5
    } +. {
      0.1
    } {
    "abc", 2.0 -> [42, 5]
    "res", _ -> []
    _, v13 -> [3]
  }
  echo fn(v14, v15) { fn(v16) { 100.0 }(False) }(2, True)
  echo export(fn(v17) { {
    0.1
  } == {
    10.0
  } }("a"))
  echo euler_value
  echo case fn(v18, v19) { euler_value }(3.14, 100.0), Cv2([]) {
    euler_value, Cv2([x, ..rest]) -> {
      "abc" <> "b"
    } <> "x"
    _, Cv2([]) -> case fn(v20) { "res" }(True) {
      "a" <> _ -> fn(v21, v22) { "" }(10, 1.5)
      a -> "b" <> a
    }
    0, Cv3("res") -> ""
    _, v23 -> "constructor"
  }
}
