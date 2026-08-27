pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2(value: Bool, inner: Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v3: Int, v4: Bool, delete: Float) -> Int {
4
}

fn f1(arguments: Int) -> Float {
case fn(v5) { [] }(100) {
    [] -> fn(v6, v7) { {
      1.5
    } -. {
      0.1
    } }(True, False)
    [arguments, ..rest] -> fn(v8) { {
      100.0
    } -. {
      1.5
    } }(7)
    _ -> {
      {
        10.0
      } *. {
        100.0
      }
    } +. {
      0.25
    }
  }
}

fn f2(v9: String) -> Bool {
{
    case 4 + 100, v9 {
      arguments, _ -> {
        let value = arguments
        let x = 0.0
        v9
      }
      v10, v11 -> "" <> v9
    }
  } != {
    case fn(v12) { 10 }("x") {
      5 -> "res"
      b -> {
        let this_ = False
        let arguments = 0.0
        v9
      }
      m -> {
        let v9 = False
        "constructor"
      }
    }
  }
}

pub fn main() {
  let value = True
  echo case <<5:16>> {
    <<_:big-unsigned-16, _:4, new:8>> -> case Cv1([3, 2], 3), [3, 7] {
      Cv1([h, a, ..], 9), [8, 8, ..] if h % 2 == 0 -> {
        let z = "bc"
        let x = value
        [100]
      }
      Cv1([], _), [3, 6, ..] -> fn(v13) { [] }("abc")
      Cv1([3], 7) as whole, [2, _, ..] -> {
        let arguments = "constructor"
        []
      }
      _, _ -> [7]
    }
    <<"bc":utf8>> -> case 2 {
      6 -> [5, 5]
      _ -> {
        let delete = "data"
        let value = [2]
        value
      }
    }
    _ -> [2, 3]
  }
  echo {
    "res" <> {
      "data" <> "ab"
    }
  } <> "a"
}
