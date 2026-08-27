pub type V0 {
  Some(value: String, inner: List(Int))
  Cv1
  Cv2(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v3: Bool) -> String {
{
    case Cv1 {
      _ -> "x"
      item -> {
        let v3 = [1, 100]
        let self_ = []
        "constructor"
      }
    }
  } <> {
    case 10 {
      item -> "constructor"
      constructor -> "ab"
    }
  }
}

fn f1(v4: Int, delete: Int) -> Float {
{
    0.0
  } +. {
    {
      let value = {
        let pair = False
        let item = pair
        True
      }
      fn(v5) { 0.0 }("abc")
    }
  }
}

fn static(v6: Int, delete: V0, new: #(List(Int), String)) -> List(Int) {
[3, 5]
}

pub fn main() {
  echo 10.0
  echo {
    case fn(v7) { [7] }(0.5), 1.5 {
      [], 1.0 as whole -> 100
      [constructor, ..rest], n -> constructor
      [6], 100.0 -> 7
      v8, v9 -> 5
    }
  } > {
    {
      let z = []
      7
    }
  }
  echo {
    {
      10 > 42
    } || True
  } || False
}
