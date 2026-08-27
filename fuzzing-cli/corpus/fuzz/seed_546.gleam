pub const seed_value: String = "abc"
pub const pi_value: Int = 5

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, rest: Int, this_: Float) -> Bool {
{
    case <<"res":utf8, "x":utf8, "bc":utf8>> {
      <<_:1, _:utf8>> -> fn(v0) { 0.5 }("a")
      <<_:utf8>> -> fn(v1) { v1 }(0.5)
      <<pair:big-unsigned-1, 3:8, _:little-unsigned-1>> -> {
        let delete = "x"
        let prototype = rest
        this_
      }
      v2 -> 1.5
    }
  } == {
    {
      let this_ = "bc" <> "bc"
      let self_ = [4]
      0.1
    }
  }
}

fn f1(pair: Bool, v: Int, default: String) -> Bool {
{
    case 10 + v {
      5 | 4 -> 10.0
      a -> 3.14
    }
  } == {
    fn(v3) { {
      1.5
    } -. {
      3.14
    } }("constructor")
  }
}

fn yield(this_: String) -> String {
this_
}

pub fn main() {
  echo case fn(v4, v5) { [] }(5, 5) {
    [0] -> [42, 3]
    [_, ..rest] -> rest
    _ -> case [] {
      [2, _, ..] -> []
      [9] -> []
      [h] -> [10]
      v6 -> {
        let x = [2]
        x
      }
    }
  }
  echo {
    let prototype = [42]
    True
  }
}
