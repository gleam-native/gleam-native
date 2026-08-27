pub type Map {
  Cv0(value: String, inner: List(Int))
}

pub type Symbol {
  Cv1(value: Int)
  Cv2(value: List(Int))
}

pub type Record {
  Cv3(Bool)
  Some(value: Int)
  Cv4(Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(value: String) -> Bool {
case value {
    inner -> case True || False {
      _ -> "data" != "x"
      False as whole -> {
        let class = whole
        let value = 1
        whole
      }
      True -> {
        let n = 42
        let self_ = "b"
        False
      }
    }
    _ -> True
    b -> case fn(v5) { 1 }("") {
      constructor -> {
        let s = 10.0
        False
      }
      5 | 2 -> {
        let rest = []
        False
      }
    }
  }
}

fn f1(v6: Float, length: String, v7: Int) -> String {
length <> {
    case "constructor" <> length {
      a -> fn(v8, v9) { a }(True, "abc")
      inner | "res" <> inner -> "constructor" <> inner
      _ -> length
    }
  }
}

pub fn main() {
  let item = 3
  let new = f1({
    0.1
  } *. {
    0.25
  }, "constructor", spin(3, 1))
  echo fn(v10, v11) { {
    {
      let item = v10
      let x = []
      new
    }
  } == f1(3.14, new, item) }(False, "x")
  echo True && {
    spin(3, item) != 5
  }
  echo new <> {
    fn(v12, v13) { "b" }(3.14, 0.25)
  }
}
