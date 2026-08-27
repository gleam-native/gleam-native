pub const tag_value: Int = 100

pub type V0 {
  Cv1
  Cv2
  Cv3(Float)
}

pub type Record {
  Cv4(Bool, List(Int))
  Number(Int, Int)
}

fn f0(self_: List(Int), v5: Bool) -> List(Int) {
[5, 2]
}

fn f1(v6: Float, v7: String) -> String {
case {
      let v7 = "res"
      Number(10, 2)
    } {
    Number(0, _) -> v7
    Cv4(True, [4, _, ..]) | Cv4(_, _) -> fn(v8) { fn(v9, v10) { v7 }(0.5, 2.0) }(3)
    v11 -> "bc"
  }
}

pub fn main() {
  let prototype = case "" <> "b", [2, 5] |> f0(2 >= tag_value) {
    "abc", [] -> 4
    default, [7, ..rest] as whole -> {
      let length = False
      let self_ = length
      4
    }
    v12, _ -> 0
  }
  let prototype = "bc" <> "data"
  echo case prototype <> prototype {
    "bc" -> True
    "constructor" <> rest -> case {
        let constructor = True
        rest
      } {
      _ -> True
      _ | "a" <> _ -> {
        1.5
      } != {
        1.0
      }
    }
    _ -> case Cv1 {
      b -> False && False
      Cv3(_) -> False
      Cv2 -> "bc" != prototype
    }
  }
  echo tag_value
}
