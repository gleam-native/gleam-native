pub const limit_value: Bool = True
pub const euler_value: Int = 42

pub type V0 {
  Some(value: String, inner: Bool)
  Cv1
  Cv2
}

fn new(v3: Float, arguments: Int) -> List(Int) {
case "b" {
    "a" as whole -> []
    "data" -> case <<1:8, "x":utf8>>, 0 - 1 {
      <<_:utf8>>, 3 -> {
        let self_ = "x"
        []
      }
      <<100:8>>, v4 if v4 > 7 -> []
      <<_:utf8>>, _ -> [1, 0]
      v5, _ -> {
        let arguments = [2, 10]
        arguments
      }
    }
    _ -> case 3, Cv2 {
      _, Cv1 -> []
      0, Cv1 -> [2]
      _, _ -> []
    }
  }
}

pub fn main() {
  let l = case "res" <> "abc" {
    constructor -> fn(v6) { 0.1 }(False)
    "abc" <> item if item == "data" || item == "res" -> {
      2.0
    } *. {
      0.1
    }
    "bc" <> rest -> 0.5
  }
  let v = case 3, fn(v7) { Cv1 }(3) {
    2, v8 -> True
    _, Cv1 -> {
      let default = []
      let default = euler_value
      True
    }
    v9, v10 -> fn(v11, v12) { v11 }(True, "res")
  }
  echo case Some("b", True), "res" <> "bc" {
    s, "constructor" -> {
      fn(v13, v14) { euler_value }(0.1, True)
    } * {
      fn(v15) { euler_value }(True)
    }
    Cv1, _ -> euler_value - euler_value
    v16, v17 -> case euler_value {
      v16 -> 5
      3 | 2 -> euler_value - euler_value
    }
  }
  echo case 2 % 1, <<"res":utf8, "abc":utf8, "abc":utf8>> {
    _, <<acc:16, 10:1, "x":utf8>> as whole -> fn(v18) { {
      let v = euler_value
      [5]
    } }("b")
    0, _ -> [2, 1]
    v19, _ -> fn(v20) { [] }(True)
  }
  echo case "bc" <> "a", "bc" <> "data" {
    "constructor", "x" -> []
    "data", "a" -> case "abc" {
      inner -> fn(v21, v22) { [1] }(True, False)
      "abc" -> fn(v23, v24) { [1] }("data", False)
    }
    "a" <> _, "data" <> _ -> case fn(v25, v26) { 0 }(True, 3.14) {
      7 | 9 -> new(2.0, 42)
      _ | 1 -> [10]
      inner -> [0]
    }
    v27, _ -> [7, 5]
  }
}
