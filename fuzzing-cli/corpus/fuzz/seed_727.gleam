pub const limit_value: Bool = True

pub type V0 {
  Some(value: String, inner: List(Int))
}

pub type Record {
  Ok
  Cv1
  Record(Bool)
}

pub type Number {
  Cv2
  Number(List(Int), Int)
}

fn extends(v3: Bool, new: Float) -> Float {
case [42, 0] {
    [0, 2, ..] -> new
    [x] -> {
      new *. new
    } -. new
    _ -> {
      fn(v4, v5) { 0.5 }("", 2)
    } -. new
  }
}

fn f1(v6: Bool) -> List(Int) {
case {
      let y = "a"
      [2, 5]
    } {
    [_, 4, ..] -> [4]
    [] -> case Some("data", [1, 5]), {
        100.0
      } +. {
        1.0
      } {
      Some("abc" <> rest, [7, ..tail]), 1.0 if rest != "ab" -> []
      v6, 0.0 -> {
        let v6 = "bc"
        let m = 100
        [100, 7]
      }
      v7, _ -> [10]
    }
    v8 -> v8
  }
}

pub fn main() {
  let default = "data"
  echo 1 - {
    {
      let limit_value = {
        0.1
      } -. {
        1.0
      }
      let limit_value = {
        let new = 100
        let default = 100
        "bc"
      }
      4
    }
  }
  echo limit_value
}
