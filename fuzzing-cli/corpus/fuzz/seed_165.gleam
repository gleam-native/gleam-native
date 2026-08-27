pub type Map {
  Cv0(value: String, inner: List(Int))
}

pub type Symbol {
  Cv1(value: Float)
  Cv2(List(Int))
  Cv3(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(v4: Int, v5: Int) -> Float {
1.5
}

fn f1(value: Symbol) -> List(Int) {
case <<0:8>> {
    <<_:8>> -> [3]
    _ -> case Cv0("constructor", []), 3 + 0 {
      v6, 0 -> [100]
      Cv0("bc" <> rest as whole, [0, ..tail]), 4 if rest == "x" || whole != "b" -> {
        let pair = []
        pair
      }
      Cv0("data" as whole, []), _ -> {
        let whole = 42
        []
      }
      v7, _ -> {
        let l = []
        l
      }
    }
  }
}

fn constructor(acc: Int) -> Bool {
False
}

pub fn main() {
  let this_ = case 100 - 0, 2 {
    _, _ -> constructor(3)
    3, 0 as whole -> True
    6, 9 -> True
  }
  let class = [4] |> walk(0 - 42)
  echo "bc"
}
