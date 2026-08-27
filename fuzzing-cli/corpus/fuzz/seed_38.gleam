pub const euler_value: Float = 0.25
pub const pi_value: Float = 0.1

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, class: List(Int), n: Bool) -> List(Int) {
case 3 + 4 {
    5 -> []
    0 -> fn(v0) { fn(v1, v2) { [42, 7] }(True, False) }(True)
    _ -> case <<2:8, "":utf8>> {
      <<"a":utf8, "ab":utf8>> -> class
      <<"":utf8, _:big-signed-8, "data":utf8>> -> [42]
      _ -> []
    }
  }
}

fn new(self_: Int, y: List(Int), v3: Float) -> List(Int) {
y
}

fn f2(s: Float, v4: Bool) -> Int {
walk(case "bc", {
      let length = []
      let delete = 0.0
      "x"
    } {
    "res", this_ if this_ == "ab" && this_ != "bc" -> new(0, [7, 4], 1.5)
    "abc" <> rest, _ -> [3]
    _, _ -> fn(v5) { [] }(5)
  }, {
    5 * 1
  } - {
    2 + 5
  })
}

pub fn main() {
  echo {
    let euler_value = case {
        let item = []
        3
      } {
      3 -> new(3, [], euler_value)
      v6 -> []
    }
    case {
        1.5
      } *. pi_value, <<"b":utf8, "constructor":utf8, "a":utf8>> {
      v7, <<_:utf8, _:16>> -> f0(3, [100, 2], True)
      v8, _ -> []
    }
  }
}
