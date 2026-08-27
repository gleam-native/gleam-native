pub const pi_value: String = "data"
pub const seed_value: Bool = False
pub const tag_value: Float = 2.0

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: String, v0: Float, v1: Bool) -> Float {
{
    let v0 = v0
    {
      let v1 = v0 +. {
        1.5
      }
      let x = 1.0
      fn(v2, v3) { 3.14 }("data", 1.5)
    }
  }
}

fn f1(arguments: Int, pair: Bool, acc: Int) -> Bool {
case {
      let s = [5, 4]
      acc
    } {
    5 -> case [] {
      [2, ..rest] -> True
      [2] -> pair
      [_, ..rest] -> !True
      v4 -> pair
    }
    a -> pair
    _ -> case <<"b":utf8>> {
      <<pair:8, _:big-unsigned-16, _:big-unsigned-8>> if pair % 2 == 0 || pair <= 6 -> {
        let prototype = "x"
        False
      }
      <<7:16>> -> {
        let new = pair
        let pair = []
        new
      }
      v5 -> True
    }
  }
}

fn f2(this_: Bool, v6: Bool) -> Int {
{
    fn(v7) { [1, 5] }("constructor")
  } |> walk(3 - 10)
}

pub fn main() {
  let delete = 0
  echo seed_value
  echo case <<"a":utf8>> {
    <<2:1, _:utf8>> -> case 0 < delete, "data" <> pi_value {
      _, _ -> [3]
      True, "a" -> []
    }
    <<7:8, self_:8>> -> []
    _ -> case delete, 2.0 {
      _, 3.14 -> []
      6 as whole, 1.0 -> []
      _, v8 -> [5, 0]
    }
  }
  echo {
    let arguments = case 5, [100, 0] {
      y, [] -> seed_value
      _, [x, constructor, ..] -> False
      _, _ -> False
    }
    let m = fn(v9, v10) { {
      let new = 0
      []
    } }("res", "abc")
    m
  }
  echo {
    let pi_value = {
      0 % 6
    } == {
      seed_value |> f2(True)
    }
    let seed_value = case "b" <> "ab" {
      "bc" <> inner -> {
        let delete = 0.25
        inner
      }
      "" <> _ | "a" -> "ab"
      v11 -> v11 <> v11
    }
    walk([3], 0 - 2)
  }
}
