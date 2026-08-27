pub const tag_value: Float = 0.1
pub const pi_value: Float = 0.0
pub const euler_value: Bool = True

pub type Map {
  Record
}

pub type V0 {
  Cv1
  Error(value: List(Int), inner: Float)
  Cv2(String, value: String)
}

pub type V3 {
  Ok(value: Int)
  Cv4
  Cv5(value: Float, inner: List(Int))
}

fn f0(v6: Int, v7: Int, arguments: Bool) -> List(Int) {
[10, 4]
}

fn f1(n: Int, self_: Float, v8: Int) -> Float {
0.0
}

fn f2(v9: Bool, delete: Int) -> String {
case Record {
    Record | Record -> "bc"
    Record -> {
      "" <> "constructor"
    } <> "constructor"
    _ -> case fn(v10) { #(False, 100.0) }("constructor") {
      #(True, _) -> "a"
      inner -> "b" <> "bc"
      item -> "ab" <> "res"
    }
  }
}

pub fn main() {
  let y = 100 |> f0(fn(v11) { 1 }(True), {
    let tag_value = "b"
    let pi_value = []
    True
  })
  echo {
    case f2(euler_value, 100), 0 {
      _, 9 -> fn(v12, v13) { 4 }("data", 0.25)
      "ab", 8 -> 4
      _, _ -> fn(v14, v15) { 1 }(2, "bc")
    }
  } - 2
  echo {
    case "data" {
      "data" -> pi_value
      "res" <> b | "b" <> b -> pi_value +. {
        0.0
      }
      _ -> pi_value +. tag_value
    }
  } +. {
    0.1
  }
  echo case Record {
    Record -> [4]
    _ -> f0(0, 100, True)
  }
  echo case 5 {
    4 | 1 -> {
      euler_value || True
    } && False
    inner -> f2(euler_value, 3) == ""
  }
}
