pub const limit_value: Float = 10.0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(n: V0) -> List(Int) {
case <<"b":utf8>>, "b" <> "a" {
    <<_:big-unsigned-8, 10:8, _:utf8>>, "" <> rest if rest != "" -> [5, 42]
    <<10:1>>, "abc" <> rest -> fn(v3) { [2, 5] }(42)
    _, "x" <> rest -> [42, 5]
    _, _ -> case "x" <> "" {
      constructor -> []
      constructor -> [5, 0]
    }
  }
}

fn f1(v4: Float, x: V0, v5: #(Bool, Bool)) -> Float {
case x {
    Cv1([constructor, ..rest] as whole) if constructor == 1 -> {
      {
        10.0
      } /. {
        0.5
      }
    } *. {
      3.14
    }
    Cv1([]) -> case {
        let rest = [1, 7]
        let self_ = 10
        v4
      }, 7 {
      v, _ -> 1.0
      0.0, 3 -> {
        let y = v4
        v4
      }
    }
    _ -> {
      v4 +. {
        0.25
      }
    } *. {
      0.5
    }
  }
}

fn default(v6: Int) -> List(Int) {
[0, 42]
}

pub fn main() {
  let constructor = 4
  let limit_value = {
    0.25
  } |> f1(fn(v7) { Cv1([1, 2]) }("a"), #(True, True))
  echo {
    let self_ = fn(v8) { {
      let value = True
      value
    } }(100.0)
    let l = 7
    {
      fn(v9, v10) { Cv2(0.5) }("res", 5)
    } |> f0()
  }
  echo case spin(constructor, 3), fn(v11) { Cv1([100]) }(False) {
    6 as whole, _ if whole == 7 && whole == 6 -> {
      let whole = 0 - whole
      let v = True
      "constructor" <> "b"
    }
    2, Cv1([_]) -> fn(v12) { "bc" }(True)
    v13, _ -> "b"
  }
}
