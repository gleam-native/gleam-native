pub type V0 {
  None(value: String, inner: Int)
  Cv1(value: String, inner: Bool)
  Some
}

pub type V2 {
  Cv3
  Cv4(List(Int))
  Cv5
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(rest: Int, v6: Int, self_: List(Int)) -> Float {
2.0
}

fn f1(v7: Bool) -> Bool {
{
    let value = case Cv4([100]), 0.25 {
      Cv4([]), 3.14 -> [7, 100]
      Cv5, 100.0 -> [5]
      _, v8 -> fn(v9, v10) { [1] }(10.0, True)
    }
    let constructor = value
    {
      fn(v11, v12) { True }(0.25, True)
    } || {
      True || True
    }
  }
}

fn f2(v13: Int, rest: #(List(Int), Float)) -> List(Int) {
[1]
}

pub fn main() {
  let pair = case "x" <> "", <<"ab":utf8, "b":utf8>> {
    "b", <<l:1, "data":utf8>> -> 10.0
    v14, <<42:8>> -> 4 |> static(3, [])
    _, _ -> {
      10.0
    } /. {
      2.0
    }
  }
  echo case 7 % 7, {
      10.0
    } +. {
      2.0
    } {
    9, 1.0 -> True
    pair, 10.0 as whole -> f1(False)
    v15, _ -> {
      let m = v15 == 4
      let prototype = 10
      fn(v16, v17) { True }("abc", 4)
    }
  }
  echo case "data", 3 + 3 {
    _, v18 -> {
      let new = {
        let pair = "a"
        [42]
      }
      let new = static(v18, v18, new)
      [2, 0]
    }
    "ab", _ -> []
    length, _ -> [42]
  }
  echo case {
      let delete = 10.0
      let class = False
      "bc"
    } {
    a -> [42, 7]
    b | "data" <> b -> {
      let default = 3 >= 5
      []
    }
    "ab" -> {
      1 % 3
    } |> f2(#([3], 3.14))
  }
  echo "b"
}
