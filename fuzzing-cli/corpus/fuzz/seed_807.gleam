pub type V0 {
  Ok(value: String, inner: Int)
  Cv1
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(n: Float, z: Float, acc: List(Int)) -> Int {
7 + 5
}

fn f1(v4: Int) -> List(Int) {
case fn(v5) { Cv1 }(False) {
    _ -> fn(v6) { [42, 10] }(0.25)
    constructor -> [42]
    b -> case "constructor" <> "ab", fn(v7) { "data" }(3) {
      "b" <> rest, _ if rest != "abc" || rest == "a" -> []
      v8, "a" -> fn(v9) { [5, 4] }(0.0)
      _, v10 -> []
    }
  }
}

fn f2(v11: List(Int), arguments: Int, l: Bool) -> Bool {
False
}

pub fn main() {
  let v = {
    let item = [1, 5]
    let item = "bc" == "data"
    [42] |> f2(3 - 4, 3 >= 5)
  }
  echo case 100 < 100, 3 {
    v, 3 -> case fn(v12, v13) { Cv3 }(3, 100), {
        1.0
      } |> f0({
        0.5
      } +. {
        0.0
      }, 2 |> f1()) {
      Cv3, _ -> [1]
      Cv3, 7 -> f1(7)
      Cv3, v -> f1(v)
      _, _ -> [5, 42]
    }
    True as whole, _ -> [1, 7]
    v14, _ -> [2]
  }
  echo {
    let v = "res"
    2
  }
  echo {
    2.0
  } -. {
    case 5, fn(v15) { Ok("x", 100) }(10.0) {
      _, Ok("a", acc) -> 10.0
      4, Ok("bc", 3) -> fn(v16, v17) { v17 }(1.0, 100.0)
      _, _ -> 1.0
    }
  }
  echo fn(v18) { {
    0.5
  } >=. {
    {
      10.0
    } +. {
      1.5
    }
  } }(0.25)
}
