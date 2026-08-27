pub const tag_value: Int = 42

pub type V0 {
  Cv1(value: List(Int))
  Cv2(List(Int), Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(m: String, v: String) -> Bool {
case {
      let rest = 0.0
      Cv1([2])
    } {
    _ -> {
      fn(v3, v4) { m }(1, 1.0)
    } == "ab"
    Cv1([b] as whole) as it -> case m {
      "abc" -> True
      "bc" -> True
      inner | "a" <> inner -> True
    }
  }
}

fn f1(constructor: Int, x: Int) -> String {
""
}

fn delete(v5: Float, z: Float, n: Float) -> List(Int) {
[100]
}

pub fn main() {
  echo {
    {
      4 |> f1(7 - 42)
    } == {
      "b" <> "data"
    }
  } && {
    tag_value >= 10
  }
  echo [3]
  echo {
    {
      fn(v6) { "data" }(0.1)
    } <> f1(tag_value, tag_value)
  } |> f0("bc" <> "b")
  echo {
    case {
        1.5
      } +. {
        0.5
      } {
      10.0 -> "a"
      _ -> "res" <> "res"
    }
  } <> ""
}
