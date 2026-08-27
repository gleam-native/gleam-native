pub const seed_value: Float = 0.0
pub const tag_value: Float = 0.1
pub const golden_value: String = "constructor"

pub type V0 {
  Error(value: String, inner: String)
  Cv1(Int)
  Some(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Float) -> List(Int) {
case "" <> "ab" {
    _ -> [2, 1]
    b | "ab" <> b -> {
      let x = v2
      let b = {
        let s = 5
        let prototype = True
        prototype
      }
      []
    }
    "bc" -> [42]
  }
}

fn export(v3: String, default: Int, s: #(Float, String)) -> List(Int) {
case {
      let s = [2]
      let v = "x"
      s
    } {
    [6, constructor, ..] if constructor <= 6 -> [2, 7]
    [constructor, ..rest] -> case fn(v4) { constructor }(2.0), 3 {
      _, s -> {
        let prototype = 0.25
        let arguments = s
        [4, 4]
      }
      _, 5 -> fn(v5, v6) { rest }(True, 0.1)
    }
    v7 -> case Some([42]), f0(3.14) {
      Some([]), [0, x, ..] if x > 9 -> v7
      arguments, [] -> {
        let self_ = "constructor"
        v7
      }
      y, [9] -> f0(3.14)
      _, _ -> []
    }
  }
}

pub fn main() {
  echo {
    let golden_value = 2.0
    case fn(v8) { True }(42), Cv1(10) {
      _, v9 -> "data"
      _, _ -> "a"
    }
  }
  echo True
  echo True
}
