pub const euler_value: Int = 100
pub const tag_value: String = "data"

pub type V0 {
  Cv1
  Cv2
  Cv3(List(Int), value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v4: Int, v5: Int, v6: Bool) -> List(Int) {
case Cv1 {
    _ -> [4, 3]
    b -> case "" {
      _ | "constructor" -> {
        let v5 = v6
        let m = 10.0
        []
      }
      "a" <> rest if rest != "" && rest != "constructor" -> []
      v6 -> [1]
    }
    Cv2 | Cv3(_, _) -> [4]
  }
}

fn f1(self_: List(Int)) -> String {
"bc"
}

pub fn main() {
  let rest = euler_value
  echo {
    1.0
  } != {
    {
      2.0
    } +. {
      fn(v7) { 0.0 }(0)
    }
  }
  echo fn(v8, v9) { case tag_value <> "ab" {
    "abc" | "res" <> _ -> fn(v10, v11) { "ab" }("b", 100.0)
    "constructor" <> rest -> rest <> rest
    _ -> "a"
  } }(5, 2)
  echo case tag_value {
    item -> case {
        3.14
      } != {
        3.14
      }, 100 + 2 {
      _, _ -> fn(v12) { [2, 100] }(10)
      _, 0 -> {
        let arguments = euler_value
        []
      }
      False, 2 -> [1]
    }
    "a" | "abc" -> [42, 100]
  }
}
