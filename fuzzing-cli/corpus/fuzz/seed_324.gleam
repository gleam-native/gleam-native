pub const euler_value: Bool = False
pub const seed_value: Int = 5

pub type V0 {
  None(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v1: String, v2: Int) -> List(Int) {
case None("", 1.5) {
    a -> [1, 100]
    None(_, item) -> case [10, 5], fn(v3, v4) { [10] }("b", True) {
      [_, _, ..], [1] -> {
        let y = False
        let x = v2
        [4, 4]
      }
      [4, ..rest], [a] -> [100, 10]
      _, v5 -> [0, 0]
    }
    _ -> case 0.25 {
      arguments -> {
        let n = [2]
        let arguments = True
        []
      }
      b -> [7]
      10.0 -> {
        let v2 = "abc"
        let s = 100
        [7]
      }
    }
  }
}

fn f1(l: Int) -> Bool {
{
    let y = "bc" != "data"
    case "res" <> "ab", <<"x":utf8>> {
      "constructor", <<_:utf8, "res":utf8>> as whole -> {
        let rest = y
        True
      }
      "data", <<"":utf8>> -> {
        let l = "b"
        let v = l
        True
      }
      "a", _ -> False
      v6, _ -> "ab" == "a"
    }
  }
}

pub fn main() {
  let seed_value = 1.0
  echo "a" == {
    "x" <> {
      "ab" <> "data"
    }
  }
  echo "a" <> {
    fn(v7) { "a" }(False)
  }
  echo "b"
}
