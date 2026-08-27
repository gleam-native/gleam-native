pub const seed_value: Int = 2

pub type Object {
  Record
  Cv0(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(v1: Object, v2: Object) -> List(Int) {
case True {
    _ -> [10]
    v2 -> []
    True -> [10, 42]
  }
}

fn f1(default: Int) -> List(Int) {
case [2] {
    [9] -> case <<"ab":utf8, "data":utf8, "res":utf8>> {
      <<1:4>> -> fn(v3) { [] }("ab")
      <<_:utf8>> as whole -> fn(v4) { [42] }("bc")
      _ -> {
        let x = True
        [1, 1]
      }
    }
    [7] -> delete(fn(v5, v6) { Record }(True, True), {
      let v = default
      Record
    })
    _ -> case "a" <> "abc", 10.0 {
      "bc", 1.5 -> []
      "" <> _ as whole, 100.0 if whole != "constructor" || whole != "res" -> {
        let whole = default
        let item = False
        []
      }
      _, 0.1 -> [0]
      _, v7 -> delete(Cv0("a"), Cv0("res"))
    }
  }
}

pub fn main() {
  let seed_value = fn(v8) { v8 }(True)
  let v = 3 * 5
  echo [7]
  echo case {
      let self_ = seed_value
      "constructor"
    } {
    "a" <> inner | "a" <> inner -> v
    inner -> v
    inner -> 4
  }
  echo {
    1.5
  } -. {
    {
      0.1
    } +. {
      {
        let seed_value = [10, 1]
        let n = seed_value
        100.0
      }
    }
  }
}
