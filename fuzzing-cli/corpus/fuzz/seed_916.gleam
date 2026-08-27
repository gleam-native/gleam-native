pub const tag_value: Int = 5
pub const euler_value: Int = 3

pub type V0 {
  Some(value: String, inner: String)
  Cv1
  Cv2
}

pub type V3 {
  Cv4(Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(this_: #(List(Int), String), v5: V3, v6: Int) -> List(Int) {
case "data" <> "data", {
      let this_ = False
      let acc = [7]
      Some("b", "b")
    } {
    "abc" <> _, Cv1 -> []
    _, _ -> case spin(2, 7) {
      2 -> {
        let default = v6
        let v5 = 1.5
        [4]
      }
      inner -> []
      l -> []
    }
    "bc" as whole, _ -> []
  }
}

fn constructor(v7: String, v8: V0) -> Float {
fn(v9, v10) { {
    {
      let n = True
      2.0
    }
  } -. {
    2.0
  } }("ab", 3)
}

pub fn main() {
  let self_ = 100
  echo case f0(#([], "a"), Cv4(0.25), self_) {
    [2, ..rest] as whole -> case True, 3 - 42 {
      False, 7 -> 2
      v, _ -> 0 + self_
    }
    [7, 9, ..] -> spin(euler_value, self_) + 2
    [] -> self_
    v11 -> 4
  }
  echo {
    0.0
  } *. {
    {
      fn(v12, v13) { v13 }(True, 1.5)
    } /. {
      0.5
    }
  }
  echo self_
}
