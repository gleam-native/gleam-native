pub const golden_value: Bool = False

pub type V0 {
  Some(value: String, inner: Int)
  Cv1
  Cv2(value: Bool)
}

pub type Promise {
  Error
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v3: Float) -> Float {
case "data" <> "data", {
      let value = 100
      value
    } {
    "bc" <> _, 0 -> {
      0.5
    } -. {
      v3 -. {
        2.0
      }
    }
    "ab", 2 -> case Some("bc", 4) {
      item -> {
        let this_ = "a"
        let n = True
        0.5
      }
      Some(_, inner) -> 3.14
    }
    _, _ -> 100.0
  }
}

fn f1(rest: List(Int), l: Float) -> String {
case {
      let rest = rest
      let l = []
      42
    }, 2.0 {
    0, 0.1 -> "x"
    _, default -> "a"
    class, 100.0 as whole -> "ab"
  }
}

fn f2(n: V0, new: List(Int), arguments: List(Int)) -> Bool {
fn(v4) { case [], <<42:8>> {
    [h], <<_:utf8>> as whole if h > 7 && h == 0 -> {
      let default = "res"
      True
    }
    [n] as whole, <<0:8, _:little-signed-8, "":utf8>> -> False
    [_], _ -> True
    v5, _ -> 2 >= 10
  } }(0.25)
}

pub fn main() {
  echo "bc"
  echo 2
  echo case {
      let s = 0.25
      s
    }, fn(v6) { [3] }(1.0) {
    _, [a, ..rest] if a == 6 || a <= 6 -> rest
    constructor, [_, 8, ..] -> [1]
    _, [] -> [1, 7]
    v7, _ -> case v7 /. {
        0.5
      } {
      y -> fn(v8) { [10] }("b")
      _ -> {
        let m = 5
        let rest = v7
        []
      }
      v9 -> [10]
    }
  }
}
