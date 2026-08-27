pub const tag_value: String = "constructor"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(arguments: Int, s: Bool) -> Float {
{
    {
      {
        0.1
      } -. {
        0.25
      }
    } -. {
      3.14
    }
  } +. {
    fn(v3) { {
      let item = [1]
      v3
    } }(2.0)
  }
}

pub fn main() {
  echo case fn(v4, v5) { [4] }(False, 3) {
    [h, _, ..] -> {
      h + h
    } == {
      fn(v6, v7) { h }("abc", 100.0)
    }
    [a, 8, ..] as whole -> fn(v8) { True }("abc")
    v9 -> case "abc" {
      inner -> {
        10.0
      } >=. {
        0.0
      }
      "constructor" <> inner -> True || True
    }
  }
  echo case "abc", fn(v10) { Cv2(100.0) }("a") {
    "b" <> rest, Cv1([6]) -> "constructor"
    _, tag_value -> "a"
    "constructor" <> rest, _ -> case [], "a" {
      [x], "b" -> "a"
      [b, ..rest], _ if b > 3 || b <= 7 -> fn(v11, v12) { tag_value }(100.0, 42)
      [0], "data" -> rest <> rest
      _, _ -> rest <> rest
    }
  }
  echo 7
}
