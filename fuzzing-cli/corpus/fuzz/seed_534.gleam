pub const tag_value: Bool = False

pub type V0 {
  Cv1
  Cv2
  Cv3(List(Int))
}

pub type V4 {
  Cv5
  Cv6
  Error(value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(z: Float, v: Bool) -> Int {
0 - {
    case z -. z {
      100.0 | 10.0 -> 1
      item -> 10 - 42
    }
  }
}

fn f1(value: Bool, class: String, prototype: #(List(Int), Float)) -> Bool {
value
}

fn f2(s: Int) -> Int {
s - 5
}

pub fn main() {
  let l = tag_value
  echo case {
      let tag_value = []
      let default = 42
      Error(7)
    } {
    _ -> {
      "" <> "constructor"
    } <> "bc"
    _ -> fn(v7) { "b" }(0.25)
    Error(7 as whole) -> {
      let self_ = "bc" == "bc"
      "a"
    }
  }
}
