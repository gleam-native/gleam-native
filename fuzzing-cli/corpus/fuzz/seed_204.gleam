pub type V0 {
  Cv1(value: List(Int))
  Cv2
}

pub type V3 {
  Number(Int, value: String)
}

pub type V4 {
  Cv5(value: Float)
  Cv6(value: Float, inner: List(Int))
  Cv7(value: Int)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn extends(v: #(Bool, Int)) -> Float {
{
    case Number(10, "") {
      Number(a, _) -> 0.25
      Number(_, "abc" <> rest) as whole -> 0.1
    }
  } *. {
    100.0
  }
}

fn f1(l: Int, rest: String, item: Int) -> String {
{
    case extends(#(False, 4)) {
      1.0 -> "data"
      a -> rest
    }
  } <> {
    case #([0], 7), {
        let m = item
        let prototype = 0.25
        [2, 3]
      } {
      #([x, 8, ..], _), [7, 7, ..] -> "constructor"
      #([0], _) as whole, [5, ..rest] -> ""
      _, v8 -> "b"
    }
  }
}

fn f2(arguments: Int) -> Int {
case 0.1, "ab" {
    arguments, item -> walk([], 1) - {
      100 * 10
    }
    _, _ -> case "a" <> "ab" {
      "res" | "res" <> _ -> arguments
      "constructor" | "ab" -> walk([7, 4], arguments)
      v9 -> fn(v10) { 42 }(10.0)
    }
  }
}

pub fn main() {
  let this_ = {
    fn(v11, v12) { #(True, 3) }(3.14, 3)
  } |> extends()
  let s = 0.0
  echo 10
}
