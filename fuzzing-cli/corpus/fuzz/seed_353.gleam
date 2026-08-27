pub const seed_value: Float = 0.1
pub const limit_value: String = "b"

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(v2: String) -> String {
{
    let pair = case {
        let this_ = [1, 10]
        #(True, 0.5)
      }, !True {
      #(_, _), item if item && !item -> [100]
      #(True, _), False as whole -> fn(v3) { [100, 3] }(0.1)
      v4, v5 -> [7, 1]
    }
    case 10.0, Cv1 {
      3.14, _ -> "abc"
      0.1 as whole, Cv1 -> "bc"
      10.0, pair -> v2
      _, _ -> "abc"
    }
  }
}

fn f1(v6: Int, m: Float, value: Int) -> List(Int) {
case <<"":utf8, "":utf8>>, value - v6 {
    <<42:16, 5:8, 100:16>>, default if default <= 7 || default <= 7 -> {
      let class = [4, 0]
      let l = {
        let v6 = class
        m
      }
      class
    }
    <<_:utf8>> as whole, _ -> []
    _, _ -> case False && True {
      a -> [4]
      _ | True -> fn(v7) { [] }(3.14)
      b -> [2]
    }
  }
}

pub fn main() {
  let limit_value = {
    42 - 7
  } + {
    fn(v8) { 42 }(False)
  }
  echo case [3], {
      let acc = ""
      let this_ = 42
      Cv1
    } {
    [], v9 -> case #(10.0, "abc") {
      #(_, "res") -> fn(v10) { 0.25 }("res")
      #(_, _) -> {
        2.0
      } +. seed_value
      _ -> seed_value -. seed_value
    }
    [1, ..rest], v11 -> seed_value
    [h, ..rest] as whole, Cv1 -> case "a" <> "b", "data" {
      _, "bc" <> _ -> {
        let n = False
        seed_value
      }
      limit_value, "x" as whole -> seed_value
      v12, _ -> {
        let delete = [3]
        let seed_value = False
        2.0
      }
    }
    v13, v14 -> case False {
      v -> 3.14
      True -> 0.25
      True | False -> seed_value
    }
  }
  echo True
}
