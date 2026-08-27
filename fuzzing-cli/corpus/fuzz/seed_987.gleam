pub const golden_value: String = "bc"
pub const euler_value: Float = 0.1

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Error(value: String, inner: String)
  Cv2(String, List(Int))
}

pub type V3 {
  Cv4(Float, value: List(Int))
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(delete: String, prototype: List(Int), l: Bool) -> Float {
{
    let self_ = case delete <> "res", <<100:8, "abc":utf8>> {
      "x", <<"abc":utf8, _:16, _:16>> -> "x"
      "constructor", _ -> "x"
      _, _ -> "x"
    }
    let x = case #(100.0, [5]), {
        let m = 0.0
        let delete = []
        m
      } {
      #(0.25, [_]), 100.0 -> []
      #(100.0, [delete, ..rest]) as whole, 0.25 -> {
        let self_ = True
        let this_ = l
        [2, 42]
      }
      _, v6 -> prototype
    }
    case x, x |> walk(10) {
      [b], 1 -> {
        3.14
      } /. {
        2.0
      }
      [], _ -> {
        0.1
      } -. {
        2.0
      }
      v7, _ -> {
        0.5
      } *. {
        1.5
      }
    }
  }
}

pub fn main() {
  let prototype = case Cv2("ab", [42, 0]), 7 {
    Cv1([], _), 0 -> 7
    Error("abc" <> _ as whole, "abc" <> rest), _ if whole != "ab" || rest != "res" -> 3 - 4
    Error("res" <> rest as whole, "constructor" <> tail), _ -> 100 * 10
    v8, _ -> {
      let v8 = [5]
      100
    }
  }
  echo f0({
    let item = prototype
    let v = item
    golden_value
  }, [], golden_value == golden_value) == {
    fn(v9) { 3.14 }(3)
  }
  echo True
  echo golden_value
}
