pub const golden_value: Int = 0

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v3: Bool) -> Int {
1
}

fn default(rest: Int) -> String {
"ab" <> {
    {
      "data" <> "b"
    } <> {
      "res" <> "x"
    }
  }
}

pub fn main() {
  let golden_value = 7
  echo 0.1
  echo {
    let acc = [2, 42]
    case "res" <> "res" {
      "x" <> item -> False
      a | "bc" <> a -> True
    }
  }
  echo 42
  echo True
}
