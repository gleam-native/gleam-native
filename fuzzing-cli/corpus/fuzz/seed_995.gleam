pub const seed_value: String = "res"
pub const limit_value: Float = 1.0
pub const golden_value: Int = 42

pub type V0 {
  None(value: String, inner: Int)
  Cv1(List(Int), value: Float)
}

fn f0(v2: Float, v3: #(Bool, Float), z: Bool) -> Int {
case fn(v4) { Cv1([], 1.5) }(0.0) {
    Cv1([4, ..rest], 0.25) -> {
      4 + 4
    } - {
      2 - 100
    }
    _ -> 4 + 4
  }
}

pub fn main() {
  let self_ = case {
      let golden_value = 3.14
      True
    } {
    inner -> {
      let inner = []
      let seed_value = 0.5
      [4, 4]
    }
    this_ -> [7, 3]
    _ | False -> [5]
  }
  echo [4, 1]
  echo !{
    {
      golden_value - golden_value
    } != f0(limit_value, #(True, 0.25), False)
  }
  echo {
    let seed_value = 100
    let v = case Cv1([10], 100.0) {
      None(_, 2) -> {
        let item = self_
        [1, 2]
      }
      None(_, b) -> []
      None(_, 6) -> self_
      v5 -> fn(v6, v7) { self_ }(100, False)
    }
    case "b" <> "res" {
      _ -> fn(v8) { v }(2)
      a | "abc" <> a -> {
        let rest = 0.1
        let a = golden_value
        [42, 0]
      }
      delete -> [0]
    }
  }
}
