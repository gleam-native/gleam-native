pub const tag_value: Int = 4

pub type V0 {
  Ok(value: String, inner: Bool)
  Cv1(value: Float, inner: String)
  Some(value: Int, inner: List(Int))
}

pub type Map {
  None(value: Float)
}

fn delete(v2: Int) -> Int {
v2
}

fn f1(v3: String, x: Int, v4: Int) -> Float {
case {
      let l = [5, 10]
      let s = "constructor"
      0.1
    } {
    b -> case False {
      True -> fn(v5, v6) { b }("abc", "a")
      _ -> 100.0
      True -> fn(v7, v8) { 100.0 }("data", 10.0)
    }
    3.14 | 100.0 -> 1.0
  }
}

pub fn main() {
  let arguments = {
    fn(v9) { 0.25 }(42)
  } *. {
    {
      10.0
    } +. {
      0.1
    }
  }
  echo "ab" <> {
    case Ok("bc", False) {
      Cv1(_, b) if b == "ab" && b != "" -> b <> b
      Some(7, []) -> "data"
      Cv1(_, "x") | Some(_, _) -> {
        let tag_value = "res"
        tag_value
      }
      _ -> fn(v10, v11) { "b" }(7, "constructor")
    }
  }
  echo case fn(v12, v13) { 0.1 }(True, "a") {
    2.0 as whole -> [0]
    item -> []
    3.14 | 0.25 -> case tag_value * 3 {
      constructor -> []
      class -> []
      _ -> [5]
    }
  }
  echo case {
      let x = "bc"
      let class = arguments
      "res"
    } {
    _ -> 0.1
    _ -> arguments
    a | "ab" <> a -> case fn(v14) { v14 }(100) {
      constructor -> arguments *. arguments
      v15 -> arguments /. {
        10.0
      }
      b -> arguments
    }
  }
}
