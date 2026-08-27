fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: Int, v0: Int, v1: Int) -> Float {
case {
      let v0 = 2.0
      constructor
    } {
    _ -> 1.5
    constructor -> 0.0
  }
}

fn static(rest: #(Int, Bool), delete: Int) -> String {
{
    let delete = {
      let delete = "b"
      "bc"
    }
    {
      delete <> delete
    } <> {
      "x" <> "bc"
    }
  }
}

pub fn main() {
  let constructor = 100.0
  echo {
    {
      10.0
    } -. constructor
  } *. f0({
    let this_ = True
    100
  }, 7 % 1, 4)
  echo case <<"a":utf8>>, "constructor" <> "abc" {
    <<_:utf8, "a":utf8>>, constructor -> case <<7:8>>, 1 - 100 {
      <<100:16>>, _ -> [42, 7]
      <<3:8, 4:1>>, 4 -> [3]
      _, 8 -> [100, 100]
      _, v2 -> [4, 5]
    }
    _, "ab" <> _ -> fn(v3) { fn(v4, v5) { [] }(True, "x") }("")
    v6, v7 -> case [] {
      [_, 7, ..] -> [7, 0]
      [3] as whole -> fn(v8, v9) { whole }(10.0, 4)
      v10 -> {
        let s = constructor
        let default = v7
        []
      }
    }
  }
  echo {
    let constructor = case 5 {
      inner -> "constructor"
      constructor -> "data"
      b -> fn(v11, v12) { "constructor" }(0.5, 100.0)
    }
    "res"
  }
  echo case fn(v13) { constructor }("res"), "res" {
    _, "constructor" as whole -> case {
        let new = True
        let l = [42]
        whole
      } {
      "ab" <> _ -> 0.1
      b -> f0(10, 0, 3)
    }
    _, _ -> {
      1.0
    } -. f0(4, 4, 2)
    _, "abc" <> _ as whole -> walk([42, 42], 5) |> f0(10 % 6, 0)
  }
}
