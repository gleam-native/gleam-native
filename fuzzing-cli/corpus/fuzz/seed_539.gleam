pub const limit_value: Bool = True
pub const golden_value: Bool = False

fn f0(m: String, v0: List(Int), v1: Bool) -> Int {
case m <> "", 42 {
    "a" <> rest, 9 if rest != "abc" -> 100
    "abc", this_ -> this_
    _, v2 -> v2
  }
}

fn f1(self_: Int, arguments: String) -> Int {
f0(case {
      let item = 0.25
      let class = item
      arguments
    }, "bc" {
    "x" <> _, "a" <> rest -> "a"
    "bc" <> rest, "abc" -> rest <> rest
    _, _ -> fn(v3) { arguments }(False)
  }, [], True)
}

fn f2(v4: Int, pair: Int) -> String {
case "b", False {
    _, v5 -> case <<"res":utf8, 10:8>> {
      <<4:8>> as whole -> "bc"
      <<default:little-unsigned-4, 2:1, _:utf8>> as whole if default % 2 == 0 -> "abc"
      _ -> "" <> "a"
    }
    "ab", s -> case {
        let v4 = s
        let pair = False
        []
      } {
      [1, _, ..] -> ""
      [] -> fn(v6, v7) { "res" }(100.0, 100.0)
      _ -> "x"
    }
  }
}

pub fn main() {
  let m = 0.0
  let limit_value = "abc"
  echo m
}
