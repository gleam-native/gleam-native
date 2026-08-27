pub const tag_value: Int = 4
pub const seed_value: Int = 5

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Some
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v4: Int, v5: Float) -> Bool {
case {
      let acc = "constructor"
      v4
    }, "constructor" {
    0 as whole, v6 -> False && True
    7, "res" <> rest if rest == "ab" && rest == "constructor" -> {
      let constructor = v5
      {
        10.0
      } <=. {
        3.14
      }
    }
    8, "ab" <> rest -> False
    _, _ -> case "bc" <> "res", "data" {
      "x", "b" -> True
      v7, "constructor" if v7 == "data" && v7 == "" -> {
        100.0
      } <=. v5
      _, "bc" as whole -> True
      v8, _ -> {
        let x = v4
        False
      }
    }
  }
}

pub fn main() {
  let acc = "ab"
  let pair = case "res" <> acc, Some {
    "abc", Some -> fn(v9, v10) { tag_value }(4, 0)
    "a", Cv1([9, ..rest], n) as whole -> tag_value - 100
    "constructor", Some -> tag_value
    _, _ -> 0
  }
  echo acc
  echo False
  echo {
    fn(v11) { {
      let rest = True
      let pair = 3.14
      "x"
    } }(4)
  } <> {
    acc <> acc
  }
}
