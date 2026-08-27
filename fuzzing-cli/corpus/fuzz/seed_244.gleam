pub const pi_value: Bool = False
pub const euler_value: Bool = False
pub const seed_value: String = "data"

pub type V0 {
  Cv1
  Cv2
}

pub type V3 {
  Cv4
  Cv5
}

pub type V6 {
  Cv7(Int, Bool)
  None(value: List(Int))
}

fn f0(n: Bool, v8: V6) -> String {
case "b", 4 {
    "a", 3 -> {
      "constructor" <> "constructor"
    } <> {
      "data" <> "data"
    }
    "x", 5 -> fn(v9) { "a" }(0.5)
    _, v10 -> case 5 == 1, fn(v11) { Cv2 }(True) {
      False, Cv2 -> "data"
      True, v8 -> "b"
      n, Cv1 -> "a" <> "a"
      v12, v13 -> "constructor"
    }
  }
}

fn f1(l: List(Int), pair: Int, v14: V3) -> Bool {
case "ab", False {
    "b", True -> case "bc" {
      "abc" <> rest | "" <> rest -> True || True
      "b" -> !False
      b -> {
        100.0
      } == {
        1.5
      }
    }
    _, True -> case "constructor" {
      b -> False
      "x" | "bc" <> _ -> {
        3.14
      } <=. {
        100.0
      }
      a -> True
    }
    "abc" <> _, True -> False
    _, v15 -> case pair {
      a -> fn(v16, v17) { v15 }("ab", 0.1)
      3 -> fn(v18, v19) { True }(4, 0)
    }
  }
}

fn new(pair: V6) -> List(Int) {
case 7 {
    7 -> case fn(v20, v21) { Cv4 }("bc", "b") {
      _ -> [10]
      Cv4 -> []
    }
    0 -> fn(v22, v23) { [3] }(False, False)
    _ -> case False, fn(v24) { 0 }(False) {
      pair, _ -> fn(v25, v26) { [] }(1, False)
      False, pair -> [0]
      True, v27 -> {
        let m = 0.5
        let x = m
        [42]
      }
    }
  }
}

pub fn main() {
  let z = fn(v28) { 1.5 }(0.25)
  let pi_value = "x"
  echo case seed_value <> pi_value {
    constructor -> {
      1.5
    } *. {
      0.5
    }
    "ab" | "x" -> case z +. {
        0.0
      }, fn(v29, v30) { seed_value }("abc", 100) {
      length, "res" <> rest if rest != "a" || length <. 2.0 -> 10.0
      _, "abc" <> rest -> 10.0
      v31, v32 -> {
        0.0
      } +. {
        100.0
      }
    }
  }
  echo {
    case 100 {
      n -> "res"
      inner -> seed_value
    }
  } == f0(True, Cv7(2, False))
  echo case 10, euler_value || True {
    v33, euler_value -> case 0.0, v33 - v33 {
      1.5, 1 -> 0.25
      2.0 as whole, 4 as it -> whole
      v34, v35 -> z
    }
    euler_value, False if euler_value > 0 -> case seed_value {
      "" <> constructor if constructor != "res" -> fn(v36) { z }(42)
      inner -> z
    }
    9, True -> 2.0
  }
  echo {
    case {
        let new = 42
        let delete = z
        #("", "b")
      } {
      inner -> seed_value
      #("data", "constructor" <> rest as whole) -> "a" <> rest
    }
  } <> pi_value
}
