pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  None
  Cv3(Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v4: String, default: Int) -> Int {
0
}

fn f1(prototype: List(Int), length: Bool, rest: Int) -> Float {
1.5
}

pub fn main() {
  echo case "bc" <> "", 100 - 0 {
    "res" as whole, _ if whole != "constructor" && whole == "data" -> "res" <> "abc"
    "bc" <> _ as whole, _ -> whole
    v5, v6 -> case "res" |> f0({
        let default = 1.0
        2
      }) {
      b -> v5
      7 as whole -> v5 <> v5
    }
  }
  echo 10 * {
    {
      {
        let pair = False
        let pair = "bc"
        7
      }
    } - walk([], 5)
  }
  echo f1(case <<"x":utf8>>, #(1.5, 0) {
    <<"data":utf8, _:16, "abc":utf8>>, #(_, _) -> []
    <<_:utf8, _:utf8>>, #(1.0, v7) if v7 > 7 -> [2]
    _, #(new, v8) -> {
      let class = 10
      let new = 4
      [5, 7]
    }
    _, _ -> fn(v9, v10) { [0, 7] }(1, True)
  }, {
    fn(v11, v12) { 0.1 }(True, 0)
  } <=. {
    0.25
  }, 42)
  echo [100]
}
