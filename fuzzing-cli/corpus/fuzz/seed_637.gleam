pub const tag_value: Int = 42
pub const euler_value: Float = 3.14
pub const pi_value: Float = 0.5

pub type V0 {
  Cv1
  Cv2
  Cv3
}

fn f0(default: List(Int)) -> String {
"res" <> "abc"
}

fn f1(s: String) -> List(Int) {
case fn(v4) { True }(True), "ab" {
    False, "b" -> case <<"":utf8, 3:8>> {
      <<constructor:8, _:utf8>> if constructor > 2 -> fn(v5) { [5, 0] }(0)
      <<_:big-unsigned-8, 4:1>> -> [100]
      v6 -> [100, 5]
    }
    v7, _ -> {
      let arguments = "bc"
      [2, 4]
    }
    _, "data" -> case False {
      True | True -> []
      b -> []
      False -> [42, 42]
    }
  }
}

pub fn main() {
  let pi_value = case "bc" |> f1() {
    [3, 4, ..] -> {
      let z = [10]
      "data"
    }
    [] -> "b"
    v8 -> {
      let tag_value = "abc"
      let y = 0.5
      "b"
    }
  }
  let pi_value = [0, 42]
  echo case "ab" |> f1() {
    [7, ..rest] -> case f0([1]) {
      _ -> True && False
      "abc" <> _ -> fn(v9, v10) { v10 }("ab", True)
      "bc" <> _ | "a" <> _ -> fn(v11) { v11 }(True)
    }
    [_] -> case #([], False), Cv2 {
      #([h, 3, ..], False), Cv1 if h > 4 -> False
      #([], False), Cv1 -> False
      #([x], False), Cv2 -> {
        1.5
      } >=. euler_value
      _, v12 -> True || False
    }
    [constructor, ..rest] -> case constructor - tag_value {
      arguments -> True
      7 | 6 -> 1 >= tag_value
      b -> False
    }
    _ -> case {
        let item = False
        item
      }, euler_value +. euler_value {
      True, 1.0 -> tag_value <= tag_value
      tag_value, _ -> fn(v13, v14) { True }(True, "a")
    }
  }
  echo {
    fn(v15, v16) { {
      let y = [5]
      let l = pi_value
      l
    } }(True, 0.5)
  } |> f0()
  echo {
    case Cv2, pi_value |> f0() {
      Cv3, pi_value if pi_value == "res" && pi_value == "res" -> fn(v17, v18) { euler_value }(0.25, 7)
      Cv3, _ -> euler_value
      _, _ -> 1.5
    }
  } +. {
    case tag_value % 1 {
      constructor -> euler_value
      a -> 3.14
    }
  }
}
