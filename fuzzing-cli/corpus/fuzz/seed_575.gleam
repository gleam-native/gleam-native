pub type V0 {
  Cv1(value: List(Int), inner: Int)
}

pub type V2 {
  Some(value: Int, inner: Bool)
  Cv3(Float)
  Cv4
}

pub type V5 {
  Cv6(value: Float, inner: Bool)
  Cv7(Float)
}

fn new(v8: Bool) -> Int {
case <<"constructor":utf8, "abc":utf8>>, {
      let pair = 100
      let y = v8
      [0, 4]
    } {
    <<constructor:8, 2:8>>, [_, ..rest] -> {
      let m = {
        let length = 5
        let prototype = rest
        rest
      }
      let z = "data"
      0 - constructor
    }
    <<"abc":utf8, delete:16>>, [] if delete > 6 && delete <= 0 -> 0
    _, [] -> 1
    v9, v10 -> {
      fn(v11, v12) { v11 }(3, True)
    } * 100
  }
}

pub fn main() {
  echo "b"
  echo case Cv6(10.0, True), [0, 4] {
    _, [b, h, ..] -> h
    Cv6(_, arguments), [a] -> new(fn(v13) { False }(3.14))
    Cv7(10.0), [2, ..rest] as whole -> 1
    v14, v15 -> case {
        let v14 = [4]
        v15
      } {
      [] as whole -> 1
      [h, ..rest] as whole -> h
      _ -> 100 + 1
    }
  }
  echo 4
}
