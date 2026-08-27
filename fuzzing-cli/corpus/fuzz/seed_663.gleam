pub const limit_value: Float = 1.0

pub type V0 {
  Cv1(value: List(Int))
  Cv2
  Number(Bool, value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(x: Int, v3: String, v4: Int) -> List(Int) {
[]
}

fn new(prototype: #(Bool, Int)) -> Float {
case "constructor" <> "res" {
    "abc" <> rest -> {
      let delete = rest <> rest
      let value = True
      {
        2.0
      } +. {
        1.0
      }
    }
    _ | "constructor" -> {
      1.5
    } /. {
      2.0
    }
    "bc" <> b -> {
      let prototype = b <> "res"
      {
        1.0
      } -. {
        0.5
      }
    }
  }
}

fn f2(z: Int, v5: String) -> Bool {
False
}

pub fn main() {
  let value = {
    let length = f2(4, "data")
    let s = 10
    "x" <> "b"
  }
  echo True
}
