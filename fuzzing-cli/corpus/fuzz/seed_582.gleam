pub const limit_value: Int = 10
pub const euler_value: Int = 100
pub const seed_value: Int = 42

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Bool, l: Int, item: Int) -> List(Int) {
case "a" <> "b", l {
    _, _ -> case "res" {
      a -> []
      "ab" | "a" -> []
    }
    _, v0 -> [7]
    "a" <> _, _ -> case walk([5, 1], 0) {
      _ -> {
        let arguments = 0.1
        let arguments = constructor
        [7]
      }
      v1 -> fn(v2, v3) { [5, 100] }(0, 1.0)
      _ -> fn(v4, v5) { [] }(False, 0.1)
    }
  }
}

fn f1(n: Int, v6: Float) -> String {
"a"
}

pub fn main() {
  let x = fn(v7) { fn(v8, v9) { [3, 42] }("x", 5) }(0.25)
  echo f1(100, case True |> f0(0, 1), limit_value * 10 {
    [1, 1, ..], 7 as whole -> fn(v10) { 0.5 }(0.25)
    [_, b, ..], 4 -> 0.5
    v11, _ -> 3.14
  })
  echo {
    let arguments = case <<"ab":utf8, "data":utf8>> {
      <<_:utf8>> -> 10.0
      _ -> {
        0.25
      } -. {
        1.0
      }
    }
    let class = {
      let l = {
        2.0
      } +. {
        10.0
      }
      let x = {
        let acc = [42, 42]
        False
      }
      False && True
    }
    fn(v12) { arguments +. {
      0.0
    } }(3)
  }
  echo x
}
