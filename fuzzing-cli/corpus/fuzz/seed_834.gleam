pub const seed_value: Float = 0.5
pub const golden_value: Bool = False
pub const tag_value: String = "constructor"

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(pair: Bool) -> Float {
{
    {
      2.0
    } -. {
      {
        2.0
      } +. {
        100.0
      }
    }
  } -. {
    {
      0.25
    } -. {
      {
        let pair = "constructor"
        let pair = 1.5
        3.14
      }
    }
  }
}

fn f1(constructor: String, l: Bool, v0: Float) -> Float {
{
    case fn(v1) { v0 }("a") {
      v -> fn(v2, v3) { v0 }(False, 0.5)
      2.0 -> 1.0
      0.5 -> v0
    }
  } -. v0
}

fn f2(x: Int, v4: String, value: Int) -> String {
{
    case "data" <> "", <<3:8, 3:4, 1:8>> {
      "constructor" <> rest, <<l:8, _:big-unsigned-4, _:big-unsigned-8>> as whole if rest != "" -> v4
      "bc", <<"abc":utf8, _:utf8, "res":utf8>> -> v4
      "res" as whole, <<"res":utf8, "x":utf8>> -> fn(v5, v6) { v4 }(True, "res")
      _, v7 -> "abc" <> v4
    }
  } <> {
    fn(v8, v9) { fn(v10, v11) { v8 }(10, 7) }("res", 3)
  }
}

pub fn main() {
  let tag_value = case "bc" {
    _ -> {
      let acc = 0
      3
    }
    seed_value | "abc" <> seed_value -> 5
  }
  let y = {
    let seed_value = [10, 42]
    seed_value
  }
  echo [7]
  echo case <<"b":utf8>>, fn(v12) { tag_value }("bc") {
    <<_:utf8, "data":utf8>>, 2 -> fn(v13, v14) { "bc" |> f1(False || golden_value, seed_value) }("", 10.0)
    <<_:utf8, 3:16>>, 0 as whole -> f1("ab", golden_value, 3.14) /. {
      10.0
    }
    _, _ -> fn(v15, v16) { f1("abc", True, 2.0) }(True, 7)
  }
}
