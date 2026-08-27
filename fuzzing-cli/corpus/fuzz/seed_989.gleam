pub const tag_value: Int = 7

pub type V0 {
  Number(value: String, inner: String)
  Cv1(value: Float, inner: String)
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v3: Int, v4: Int, z: Bool) -> String {
case {
      let default = 7
      Cv2
    } {
    b -> case z, <<"a":utf8, "a":utf8, "b":utf8>> {
      v4, <<_:utf8>> if !v4 && !v4 -> "res"
      False, _ -> "" <> "a"
      _, _ -> {
        let y = True
        let delete = 2.0
        "b"
      }
    }
    Cv2 as whole -> ""
    Cv1(_, constructor) -> case 1.5 {
      v5 -> constructor <> constructor
      10.0 -> constructor <> constructor
      100.0 -> "bc"
    }
  }
}

fn f1(v6: Bool, pair: Bool, v7: Int) -> String {
case fn(v8, v9) { 3 }(2, "a"), {
      let length = pair
      #(False, "bc")
    } {
    v10, #(False, _) -> f0(spin(v7, v10), v7, {
      let arguments = "abc"
      let n = 3.14
      True
    })
    6, #(_, "abc" <> rest) if rest != "data" -> "a"
    4, #(True, "b") as whole -> {
      {
        let delete = pair
        "x"
      }
    } <> {
      "data" <> "abc"
    }
    _, _ -> "ab" <> {
      "b" <> "data"
    }
  }
}

pub fn main() {
  echo False
  echo case 0.5 {
    0.25 -> case {
        let l = tag_value
        tag_value
      }, [] {
      z, [_] if z > 1 -> {
        let v = []
        let arguments = False
        3.14
      }
      _, [_] -> 0.25
      3 as whole, [_, _, ..] as it -> {
        10.0
      } -. {
        10.0
      }
      v11, v12 -> fn(v13, v14) { v14 }("constructor", 10.0)
    }
    _ | 0.1 -> case fn(v15) { 10.0 }(2), "" {
      2.0, "abc" <> _ -> {
        100.0
      } -. {
        0.0
      }
      2.0, l -> fn(v16) { v16 }(2.0)
      2.0, _ -> 1.0
      _, _ -> fn(v17) { 3.14 }(True)
    }
  }
  echo {
    let tag_value = "data" <> "bc"
    let value = case 100 + 100, [3, 10] {
      4, [8, constructor, ..] -> []
      1, [9, constructor, ..] -> [4, 7]
      _, _ -> [4]
    }
    False
  }
  echo 3
}
