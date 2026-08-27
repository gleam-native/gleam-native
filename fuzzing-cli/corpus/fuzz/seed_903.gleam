pub const seed_value: String = "res"

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v0: String) -> String {
"ab"
}

pub fn main() {
  let n = case {
      let value = 3.14
      4
    } {
    8 -> []
    5 -> [4]
    _ -> []
  }
  let length = case <<42:4>> {
    <<_:utf8>> -> fn(v1, v2) { n }("abc", 7)
    <<_:utf8, _:utf8>> -> n
    _ -> n
  }
  echo {
    case {
        let l = False
        0.1
      } {
      _ -> 3
      inner -> spin(4, 3)
    }
  } <= 7
  echo 3
  echo {
    {
      let seed_value = "bc" |> f0()
      seed_value
    }
  } <> {
    {
      let this_ = 100 % 5
      let pair = {
        10.0
      } +. {
        100.0
      }
      "abc"
    }
  }
}
