pub const seed_value: Int = 5
pub const euler_value: Int = 5
pub const golden_value: Float = 10.0

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: String, rest: Bool, v0: List(Int)) -> String {
{
    {
      let pair = {
        let rest = rest
        rest
      }
      let class = fn(v1) { [100] }("res")
      fn(v2, v3) { "res" }("abc", 100.0)
    }
  } <> constructor
}

fn f1(v4: Int) -> Bool {
case 0 {
    0 -> v4 > 0
    item -> True
    constructor -> False
  }
}

fn f2(v5: Bool) -> List(Int) {
case 7 > 0, "" {
    _, "data" -> {
      let new = 1 - 42
      [10, 5]
    }
    False, "bc" -> []
    False, _ -> []
    _, v6 -> case <<7:16>>, #(0.0, 1.5) {
      <<"":utf8>>, #(_, 0.1) -> fn(v7) { [100, 10] }(0.0)
      <<_:big-signed-16, "bc":utf8>>, #(_, v5) -> [5, 7]
      _, #(_, 0.0) -> [3]
      v8, v9 -> [100, 42]
    }
  }
}

pub fn main() {
  let n = case False || True {
    v10 -> seed_value |> spin(seed_value)
    True -> euler_value
    True -> {
      let s = "res"
      seed_value
    }
  }
  echo {
    let acc = case <<"data":utf8, "abc":utf8, "res":utf8>>, [] {
      <<"constructor":utf8, _:utf8, _:big-unsigned-16>>, [9, a, ..] -> f2(False)
      _, [_, ..rest] -> f2(False)
      _, v11 -> v11
    }
    {
      4 |> spin(10 + 42)
    } * {
      5 + 3
    }
  }
  echo n % 2
  echo 0.25
  echo False
}
