pub type V0 {
  Number(value: String, inner: List(Int))
  Some(Int)
  Cv1(List(Int))
}

fn f0(v2: Bool) -> Bool {
True
}

fn f1(x: String, v3: Int, n: Int) -> String {
{
    case #(42, 1), "b" <> x {
      #(_, _), "res" -> "x" <> ""
      #(_, 9), m if m != "a" || m == "a" -> m
      #(x, v4), _ -> "a"
      v5, v6 -> fn(v7, v8) { "res" }("bc", True)
    }
  } <> "bc"
}

pub fn main() {
  let y = case {
      let this_ = [3]
      [42, 5]
    } {
    [x, 4, ..] as whole -> 42
    [6] -> {
      let arguments = True
      let value = True
      3
    }
    [2, ..rest] -> 0
    _ -> 2
  }
  let this_ = {
    "ab" <> "abc"
  } <> {
    "a" <> "data"
  }
  echo {
    case 0 {
      new -> {
        let class = 1.5
        let v = this_
        4
      }
      item -> item - item
    }
  } - {
    {
      0 - 42
    } - 4
  }
  echo f1({
    fn(v9) { "" }(True)
  } <> "x", 4 + 10, fn(v10) { 42 * 4 }(False))
  echo {
    let m = fn(v11) { False }(2)
    case fn(v12, v13) { y }("x", 2.0), Cv1([3, 0]) {
      7, Some(8) -> "constructor"
      2, Cv1([_]) -> "data"
      _, _ -> {
        let arguments = [0, 42]
        let this_ = ""
        this_
      }
    }
  }
}
