pub const euler_value: String = "data"
pub const limit_value: Int = 100

pub type V0 {
  Ok(value: String, inner: Bool)
}

pub type V1 {
  Cv2
  Cv3(Int, value: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn yield(v4: Int, x: Float, v5: #(List(Int), Bool)) -> String {
fn(v6, v7) { case fn(v8) { "b" }(True), <<"a":utf8>> {
    "" <> rest, <<_:8>> -> v7
    "abc", _ -> v6 <> "abc"
    _, v9 -> {
      let s = v7
      let v = True
      v6
    }
  } }("constructor", "")
}

fn f1(v10: Int) -> Int {
case "b" {
    "a" -> case "x" <> "abc" {
      "res" | "abc" -> 100
      "b" -> {
        let prototype = True
        v10
      }
      v11 -> {
        let item = [1, 7]
        let arguments = 1.5
        v10
      }
    }
    "a" <> rest -> 5 - spin(v10, 1)
    _ -> spin(100, 7 + v10)
  }
}

fn delete(v12: Bool, v13: Int) -> List(Int) {
case [] {
    [v13] -> []
    [constructor, 6, ..] as whole -> {
      let self_ = "bc"
      whole
    }
    _ -> {
      let v12 = {
        0.0
      } *. {
        3.14
      }
      let v13 = "x"
      [100]
    }
  }
}

pub fn main() {
  let m = 0.25
  echo {
    fn(v14, v15) { {
      0.25
    } == m }(True, 2)
  } || False
}
