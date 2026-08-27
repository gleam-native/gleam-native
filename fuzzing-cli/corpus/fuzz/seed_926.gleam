pub type V0 {
  Ok(value: String, inner: Bool)
  Cv1
}

pub type Object {
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(acc: Object) -> Int {
case "" {
    "a" <> rest if rest != "bc" -> spin(4 |> spin(2), {
      let acc = 0.1
      let self_ = False
      42
    })
    _ | "ab" -> {
      7 - 2
    } * 0
    constructor | "ab" <> constructor -> {
      let l = constructor <> constructor
      let constructor = spin(7, 42)
      100 |> spin(fn(v3, v4) { 10 }(0.25, True))
    }
  }
}

pub fn main() {
  let delete = True
  let length = []
  echo length
  echo case 0 |> spin(f0(Cv2)) {
    7 -> [10]
    b -> case Ok("data", False) {
      Ok("x", False) -> []
      Ok(b, _) -> []
      _ | Ok(_, _) -> length
    }
    4 -> case 100 * 4 {
      b -> [2]
      8 -> length
      delete -> {
        let length = False
        let prototype = "b"
        [10, 7]
      }
    }
  }
  echo {
    case 100.0 {
      2.0 -> 1.5
      0.1 -> 0.1
      1.5 -> 1.0
      _ -> fn(v5) { 0.1 }("b")
    }
  } +. {
    case Cv2, 42 + 4 {
      Cv2, _ -> {
        0.1
      } -. {
        0.1
      }
      Cv2 as whole, 4 -> {
        0.5
      } +. {
        10.0
      }
      Cv2, class -> 0.25
      _, _ -> 0.25
    }
  }
}
