pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(z: Bool, rest: Bool, y: Int) -> List(Int) {
[]
}

fn default(new: Float, v2: Int) -> Float {
case <<"b":utf8>> {
    <<_:little-signed-8, _:utf8, _:big-unsigned-8>> -> 10.0
    _ -> case {
        let new = "constructor"
        let v2 = [42]
        "b"
      }, [100, 7] |> walk(1) {
      "constructor" <> _, 5 -> {
        let new = v2
        let n = 3.14
        n
      }
      "a", 4 -> new *. new
      _, 7 -> new
      v3, _ -> 2.0
    }
  }
}

pub fn main() {
  let y = {
    let pair = {
      2.0
    } -. {
      2.0
    }
    "data"
  }
  echo case "", {
      let v = y
      let constructor = 3.14
      True
    } {
    v4, True -> case v4 <> y {
      "constructor" -> [2, 1]
      _ -> [10]
      b -> [4, 1]
    }
    _, True as whole if whole && whole -> f0(0 >= 4, fn(v5) { whole }(5), [2, 4] |> walk(10))
    "res", True as whole -> fn(v6) { f0(False, False, v6) }(4)
    v7, v8 -> fn(v9) { [] }(1.0)
  }
  echo [2, 10]
  echo "abc"
}
