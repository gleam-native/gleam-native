pub const limit_value: Int = 2

pub type Number {
  Cv0(value: String, inner: List(Int))
  Some
}

pub type Object {
  Cv1
  Cv2(Int, value: String)
  Cv3
}

pub type V4 {
  Cv5(Bool, value: String)
  Number(Float, value: Float)
  Cv6(String)
}

fn f0(self_: #(Bool, List(Int)), v7: Float, pair: Bool) -> Float {
v7
}

fn f1(item: Bool, m: Int, rest: Bool) -> List(Int) {
case Cv0("ab", [100]) {
    inner -> {
      let inner = 100.0
      let length = fn(v8, v9) { 3 }(True, True)
      []
    }
    Cv0(v10, [a, ..rest]) -> [0]
    inner -> [1]
  }
}

pub fn main() {
  let default = {
    let acc = "res"
    limit_value - 4
  }
  let limit_value = case {
      let delete = default
      let s = default
      Cv0("a", [2])
    }, [3] {
    _, [6, 4, ..] -> default
    Cv0("" <> _, [a, ..rest]) as whole, [] -> fn(v11, v12) { limit_value }("ab", "b")
    v13, _ -> default
  }
  echo case Cv3 {
    Cv3 -> 4 * {
      10 + limit_value
    }
    item -> case [] {
      [x, 1, ..] if x <= 5 -> 1
      [a, ..rest] -> {
        let limit_value = "constructor"
        default
      }
      [3] -> 2
      v14 -> default
    }
  }
  echo case <<"b":utf8>> {
    <<4:16>> -> "bc" <> "b"
    _ -> case fn(v15) { Cv3 }(True), {
        let acc = 10.0
        let x = [3]
        "constructor"
      } {
      Cv2(constructor, "constructor"), "a" if constructor <= 0 && constructor % 2 == 0 -> "ab"
      Cv1 as whole, "x" -> "res" <> "data"
      _, "constructor" <> rest -> rest
      _, v16 -> fn(v17) { v16 }(False)
    }
  }
}
