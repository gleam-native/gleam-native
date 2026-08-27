fn static(constructor: List(Int), v0: Int, v1: Bool) -> Float {
{
    {
      let z = [5]
      let y = {
        let m = 10.0
        v0
      }
      fn(v2) { 3.14 }("a")
    }
  } -. {
    {
      {
        0.5
      } *. {
        1.0
      }
    } +. {
      2.0
    }
  }
}

fn f1(delete: String, arguments: Bool) -> String {
delete
}

fn f2(s: Bool, v3: List(Int)) -> String {
case "" <> "constructor", 1 % 1 {
    _, 2 -> "b" <> "bc"
    "data" <> rest, v4 if v4 % 2 == 0 && v4 > 7 -> case v4 - v4 {
      4 -> "ab"
      inner -> "bc"
      n -> fn(v5, v6) { rest }(1, 0.1)
    }
    v7, v3 -> {
      "x" <> v7
    } |> f1({
      let new = 1.5
      s
    })
  }
}

pub fn main() {
  let s = case True, 100.0 {
    False, 1.5 -> 100
    v8, v -> fn(v9, v10) { 0 }(True, 1.0)
  }
  let constructor = case True |> f2([100, 42]) {
    "" <> constructor if constructor != "bc" -> False
    s -> False
  }
  echo {
    7 * 42
  } - {
    {
      s * s
    } - 1
  }
  echo case 100.0 {
    a -> {
      let prototype = [100]
      a
    }
    a -> case 4 {
      5 -> static([], s, constructor)
      b -> 1.5
      7 -> a -. {
        0.1
      }
    }
  }
  echo case 1 + s, False {
    _, constructor -> case <<100:16, 2:4, "abc":utf8>> {
      <<"constructor":utf8, _:utf8>> -> [1]
      _ -> [42, 1]
    }
    _, False -> case True {
      constructor -> fn(v11, v12) { [2, 10] }(True, "res")
      a -> []
      item -> [2, 7]
    }
    3, _ -> case 10.0 {
      1.5 | 1.0 -> {
        let new = [4, 42]
        let class = [42]
        class
      }
      10.0 -> [4, 5]
      10.0 | 1.0 -> {
        let s = 100.0
        [100]
      }
      v13 -> {
        let s = True
        []
      }
    }
  }
}
