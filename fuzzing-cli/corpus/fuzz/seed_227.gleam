pub type V0 {
  Ok(value: String, inner: Int)
}

fn f0(v1: V0) -> List(Int) {
[42, 0]
}

pub fn main() {
  let s = case 42 + 7, "x" <> "abc" {
    2, "data" -> []
    4, v2 -> {
      let v = 10.0
      [3]
    }
    _, _ -> f0(Ok("a", 1))
  }
  let s = {
    "a" <> "b"
  } <> {
    "b" <> "abc"
  }
  echo 0.5
  echo {
    let constructor = "bc"
    let self_ = []
    case 0.25, [10, 4] {
      10.0, [_] -> True
      l, [3] -> {
        let prototype = s
        let x = 0
        False
      }
      _, v3 -> True
    }
  }
  echo [1]
}
