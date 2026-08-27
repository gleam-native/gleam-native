pub const limit_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Record(Float)
  Ok
}

pub type V2 {
  Cv3
}

fn constructor(length: Bool, v4: V2) -> List(Int) {
[]
}

fn f1(y: Bool) -> String {
case Cv3 {
    Cv3 -> {
      let y = 42 == 2
      "abc"
    }
    v5 -> "res"
    y -> case "constructor", Ok {
      _, Cv1([y], 0) if y <= 7 -> "a"
      "abc", Ok -> "res" <> "ab"
      _, Cv1([y], 1) -> "constructor" <> "abc"
      _, _ -> "bc" <> "a"
    }
  }
}

pub fn main() {
  let default = {
    let limit_value = True
    let l = True && limit_value
    limit_value
  }
  echo "abc"
}
