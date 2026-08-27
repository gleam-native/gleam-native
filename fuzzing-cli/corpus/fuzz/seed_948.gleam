pub type V0 {
  Cv1
  Cv2
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(l: String, arguments: List(Int), y: V0) -> Bool {
!True
}

fn extends(z: V0, l: Int, v4: List(Int)) -> Bool {
False
}

fn delete(v5: List(Int), v6: #(List(Int), Float)) -> Int {
0 % 6
}

pub fn main() {
  let x = []
  let n = case {
      let x = True
      let rest = []
      42
    }, 0 * 10 {
    3, 7 -> {
      let x = True
      let item = 42
      0.5
    }
    v7, x -> 0.1
    4, v8 -> 1.5
  }
  echo False
  echo fn(v9, v10) { "bc" <> {
    fn(v11) { "x" }(True)
  } }(1.5, 7)
  echo {
    fn(v12) { [7, 7] |> walk(42) }("data")
  } < 7
}
