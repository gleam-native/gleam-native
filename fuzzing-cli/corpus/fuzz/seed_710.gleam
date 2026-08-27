pub type Object {
  Cv0(value: String, inner: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: Int, prototype: List(Int), v2: Int) -> List(Int) {
case Cv0("bc", [5, 5]) {
    constructor -> case [] {
      [constructor, ..rest] if constructor > 4 || constructor == 2 -> []
      [_, _, ..] -> prototype
      [x, ..rest] -> rest
      _ -> {
        let v2 = 3.14
        prototype
      }
    }
    Cv0(v3, [9, ..rest]) -> []
  }
}

fn f1(v4: List(Int)) -> Float {
1.5
}

fn yield(v: List(Int), v5: String) -> Float {
0.1
}

pub fn main() {
  let item = True
  echo case 0.5 {
    constructor -> False
    2.0 -> {
      {
        let self_ = "data"
        3.14
      }
    } == {
      0.1
    }
    a -> False
  }
  echo case {
      let item = "b"
      10
    } {
    v6 -> case <<5:8, 5:1>> {
      <<4:4>> -> 3 + v6
      <<4:16>> -> v6 + v6
      _ -> fn(v7) { v6 }("")
    }
    b -> fn(v8) { 3 + 0 }("abc")
  }
  echo "a"
}
