pub const golden_value: Int = 10
pub const tag_value: Float = 0.5
pub const euler_value: Int = 7

pub type V0 {
  Cv1
  Cv2
  Cv3(String, value: Bool)
}

fn f0(acc: #(List(Int), Int), self_: V0) -> Bool {
fn(v4, v5) { False }(0.25, 3)
}

fn f1(v6: Int, value: String) -> String {
"a"
}

fn f2(arguments: V0, item: Bool) -> Bool {
case Cv1, 100 + 7 {
    Cv1, v7 if v7 <= 4 -> item
    Cv3("x", v8), 3 if v8 || !v8 -> False
    Cv2, 2 -> case [10, 42], "data" {
      [0, ..rest], "x" -> item || item
      [5] as whole, "bc" -> item || True
      [constructor, ..rest], "bc" <> _ -> False
      v9, _ -> 0 >= 100
    }
    v10, _ -> True
  }
}

pub fn main() {
  let value = f0({
    let z = [42, 0]
    let prototype = "a"
    #([], 0)
  }, {
    let y = euler_value
    Cv1
  })
  let golden_value = case 7 {
    _ -> {
      let prototype = True
      euler_value
    }
    a -> 3 - 5
  }
  echo "" <> "x"
  echo tag_value *. {
    case Cv2 {
      b -> tag_value
      Cv3(a, _) if a == "a" && a == "data" -> tag_value +. {
        2.0
      }
      item -> tag_value /. {
        2.0
      }
    }
  }
  echo 0.0
  echo value
}
