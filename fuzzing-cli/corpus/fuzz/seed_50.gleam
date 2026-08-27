pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(v3: String, v4: String, pair: V0) -> Float {
case True, 0 * 10 {
    _, _ -> case 10, fn(v5) { [4] }(2.0) {
      9, [] -> 0.25
      _, [] as whole -> 3.14
      v6, _ -> {
        1.0
      } -. {
        0.0
      }
    }
    False, 4 -> case "res" <> v3 {
      a -> 0.1
      v4 | "data" <> v4 -> 0.1
    }
    v7, 5 -> {
      0.0
    } *. {
      {
        0.25
      } -. {
        0.1
      }
    }
  }
}

pub fn main() {
  echo [4, 7]
  echo case <<10:4>> {
    <<5:1>> as whole -> case "a", <<0:1, "a":utf8>> {
      _, <<_:utf8, _:utf8>> -> [0, 42]
      "constructor", <<"constructor":utf8>> -> fn(v8, v9) { [] }("", True)
      v10, v11 -> [1, 1]
    }
    <<_:utf8>> -> [4]
    _ -> []
  }
  echo False
}
