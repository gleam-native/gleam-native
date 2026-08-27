pub const tag_value: Bool = False
pub const pi_value: Bool = True
pub const limit_value: Int = 1

fn f0(m: String) -> Bool {
case 3, <<4:8, "ab":utf8, "res":utf8>> {
    v0, <<2:4, "":utf8, "data":utf8>> -> True
    v1, _ -> !True
  }
}

fn f1(s: String, rest: #(Int, String)) -> Bool {
case {
      let l = s
      let default = 2
      True
    } {
    False as whole -> {
      "x" <> s
    } != s
    constructor -> "b" |> f0()
  }
}

fn f2(item: Bool) -> Bool {
f0("constructor")
}

pub fn main() {
  let value = 0 * 7
  let default = case 0 {
    inner -> "" <> "x"
    _ -> "data" <> "data"
    n -> "constructor" <> ""
  }
  echo [2, 7]
  echo {
    {
      let this_ = {
        let new = [10]
        1.0
      }
      value
    }
  } + {
    case 100, default <> default {
      _, _ -> fn(v2, v3) { 2 }("ab", False)
      9, "res" -> value
      0, "a" -> limit_value
    }
  }
}
