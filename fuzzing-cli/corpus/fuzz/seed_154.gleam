pub type V0 {
  Number(value: String, inner: String)
  Error(value: Float)
}

pub type V1 {
  Cv2(List(Int), Int)
  None(Float, value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(item: V0, v3: Float) -> Int {
42
}

pub fn main() {
  let n = !False
  let n = 3.14
  echo ""
  echo "res"
  echo "a"
  echo {
    case {
        let this_ = 4
        let l = 0.5
        #("ab", [])
      } {
      #(_, [8, x, ..]) if x > 3 -> {
        let s = []
        0.0
      }
      inner -> n
      #("b", [4, ..rest]) -> {
        1.5
      } -. n
    }
  } -. {
    {
      0.1
    } -. {
      fn(v4) { n }("b")
    }
  }
}
