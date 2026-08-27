pub type Map {
  Record
  Cv0(value: List(Int))
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: String) -> Float {
fn(v3) { case 3.14, Record {
    0.1, Record as whole -> {
      let whole = [10, 5]
      let v3 = 42
      0.0
    }
    0.5, Cv1 -> {
      0.0
    } /. {
      1.0
    }
    3.14, _ -> 0.1
    _, _ -> 1.0
  } }("b")
}

fn f1(v4: Map, arguments: List(Int), v5: Int) -> Int {
0 - {
    case True, fn(v6) { #(False, False) }(7) {
      True, #(False, True) -> {
        let constructor = False
        7
      }
      _, #(self_, v7) -> 7
      _, #(False as whole, True) -> fn(v8) { v5 }(1.5)
      _, v9 -> 4
    }
  }
}

fn f2(delete: Int, new: Bool, v10: Int) -> Float {
{
    0.5
  } -. {
    f0("constructor") -. f0("bc")
  }
}

pub fn main() {
  let prototype = case {
      10.0
    } <. {
      0.1
    } {
    True -> [2, 10]
    _ -> []
    a -> []
  }
  echo case {
      let prototype = True
      let z = []
      z
    } {
    [3, ..rest] -> 1
    [a, ..rest] -> fn(v11) { f1(Cv1, prototype, 100) }("abc")
    v12 -> {
      let this_ = {
        let m = True
        let item = 2
        prototype
      }
      walk(v12, 0)
    }
  }
  echo {
    {
      0 |> f2(1 == 4, Cv1 |> f1(prototype, prototype |> walk({
        let v = 1
        7
      })))
    } +. {
      {
        let prototype = [2]
        0.0
      }
    }
  } >. {
    {
      10.0
    } *. f0("bc")
  }
}
