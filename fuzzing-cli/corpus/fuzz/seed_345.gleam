pub const pi_value: Bool = False
pub const tag_value: Float = 3.14

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Error
}

pub type Promise {
  Number(List(Int), value: Int)
  Cv2(Bool)
  Cv3(value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v4: String) -> Int {
4
}

fn f1(this_: List(Int)) -> Int {
5
}

pub fn main() {
  let this_ = {
    tag_value +. tag_value
  } -. {
    tag_value +. {
      2.0
    }
  }
  echo ""
  echo case fn(v5, v6) { 2 }(1.5, 3.14), walk([], 5) {
    4, v7 if v7 % 2 == 0 || v7 == 5 -> []
    7, 9 -> case tag_value -. this_ {
      _ -> [3, 0]
      a -> []
      2.0 -> []
    }
    0, 3 -> case "abc", 4 {
      _, item -> [5]
      _, _ -> fn(v8, v9) { [7, 3] }("bc", "b")
    }
    v10, _ -> {
      let rest = v10
      let delete = this_ != this_
      {
        let arguments = [42, 3]
        let prototype = "abc"
        []
      }
    }
  }
  echo case [4], 3 {
    [], _ -> case [3, 7] {
      [_] -> "res" <> "constructor"
      [constructor] as whole if constructor <= 9 -> "abc"
      [a, ..rest] -> "abc" <> "bc"
      _ -> "x"
    }
    [], v11 -> {
      {
        let v11 = [10, 10]
        let pi_value = "ab"
        "b"
      }
    } <> {
      {
        let length = v11
        "ab"
      }
    }
    _, _ -> "x"
  }
  echo case <<"abc":utf8, 3:16, "bc":utf8>> {
    <<_:utf8>> -> [100, 10]
    _ -> [1, 0]
  }
}
