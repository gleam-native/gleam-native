pub const pi_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(value: Bool)
  Cv4
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(value: List(Int), default: #(String, Bool)) -> List(Int) {
case "", 42 - 0 {
    "b", v5 -> []
    "ab", default -> fn(v6) { {
      let v6 = False
      let class = value
      class
    } }(4)
    "abc" as whole, default -> [100, 7]
    v7, _ -> []
  }
}

fn f1(new: Int, v8: V0, v9: Bool) -> String {
"res"
}

pub fn main() {
  let x = {
    "ab" <> "bc"
  } <> {
    "" <> ""
  }
  let x = True
  echo fn(v10, v11) { case Cv4 {
    b -> {
      1.5
    } -. pi_value
    Cv3(_) as whole -> 1.5
    a -> pi_value /. {
      3.14
    }
  } }(False, True)
  echo {
    let y = fn(v12) { f0([], #("ab", False)) }(False)
    let s = {
      let default = [1]
      {
        let length = y
        let length = "b"
        x
      }
    }
    {
      4 - 1
    } >= spin(3, 3)
  }
  echo {
    let s = case pi_value -. {
        3.14
      } {
      constructor -> {
        let delete = 0.5
        let m = "b"
        [5]
      }
      b -> {
        let pi_value = "res"
        let x = b
        []
      }
    }
    let m = pi_value *. {
      0.0
    }
    "ab"
  }
  echo [1, 3]
}
