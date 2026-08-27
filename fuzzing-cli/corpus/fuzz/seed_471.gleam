pub const euler_value: Int = 7
pub const golden_value: Int = 10

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(value: List(Int), inner: List(Int))
}

pub type Number {
  Cv4(value: List(Int), inner: Int)
  Cv5(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(s: Float) -> List(Int) {
{
    let delete = True
    []
  }
}

fn f1(v: Bool, delete: Int, v6: Bool) -> Int {
walk([2, 3], fn(v7) { delete }(1.0)) - {
    case #("a", "bc") {
      #(_, "" <> rest as whole) -> 10 % 7
      #(new, "x") -> 2
      _ -> delete
    }
  }
}

pub fn main() {
  let self_ = case {
      let this_ = 2
      Cv1
    } {
    a -> False
    Cv1 -> True
  }
  let acc = [4]
  echo case "ab" {
    a -> self_
    "constructor" <> rest if rest == "ab" || rest == "constructor" -> False
    "" <> rest | "res" <> rest -> fn(v8, v9) { self_ }("constructor", 3.14)
  }
  echo case "bc" {
    constructor -> {
      let v = True
      let class = []
      self_
    }
    "x" | "ab" -> True
    constructor | "" <> constructor -> True
  }
  echo [2, 4]
  echo {
    100.0
  } +. {
    100.0
  }
}
