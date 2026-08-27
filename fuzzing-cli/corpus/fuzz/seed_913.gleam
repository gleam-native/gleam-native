pub const euler_value: Float = 2.0
pub const golden_value: Bool = False

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(String, Float), z: #(Int, Float), self_: List(Int)) -> Float {
case "ab", "a" <> "data" {
    "bc", _ -> 1.0
    "abc" <> rest as whole, "x" <> _ as it -> 1.5
    _, v0 -> {
      1.5
    } +. {
      1.5
    }
  }
}

pub fn main() {
  echo case 0, <<"bc":utf8, "data":utf8>> {
    pair, <<_:big-unsigned-16, _:utf8>> as whole -> case "res" <> "res", euler_value -. {
        100.0
      } {
      _, 100.0 -> [5]
      _, _ -> fn(v1) { [1, 100] }(100)
      "" <> rest as whole, 10.0 -> [7]
    }
    v2, _ -> case v2 + v2, "ab" <> "a" {
      golden_value, "data" -> [0]
      _, v3 -> [10]
    }
  }
  echo case <<100:8, "data":utf8>> {
    <<_:utf8>> -> fn(v4) { euler_value }(7)
    <<100:4>> -> 100.0
    v5 -> {
      0.1
    } *. {
      0.1
    }
  }
}
