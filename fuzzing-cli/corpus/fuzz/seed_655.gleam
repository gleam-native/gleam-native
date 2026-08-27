pub const euler_value: String = "b"
pub const seed_value: Bool = False

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v2: Int, length: #(Bool, List(Int)), v3: V0) -> List(Int) {
case {
      let v = True
      let constructor = [10, 2]
      v3
    }, v2 == 0 {
    Cv1, False -> fn(v4) { [7, 4] }("x")
    Cv1, True -> []
    v5, v6 -> case "b" {
      "ab" -> [7]
      item -> [2, 5]
      n -> {
        let s = v6
        [7]
      }
    }
  }
}

fn f1(y: Int) -> Int {
case 1.0, "a" {
    _, _ -> walk([3], [5] |> walk(walk([0], y)))
    0.25 as whole, "constructor" <> _ -> walk(fn(v7, v8) { [] }(10.0, 100), y)
    1.0, "abc" -> case fn(v9, v10) { Cv1 }("x", "constructor") {
      _ -> y + 1
      Cv1 | Cv1 -> [10] |> walk(fn(v11) { y }(False))
    }
  }
}

pub fn main() {
  let item = False
  echo case "b", f1(0) {
    "res", _ -> case Cv1 {
      Cv1 | Cv1 -> {
        let item = "x"
        let seed_value = seed_value
        0.1
      }
      Cv1 | Cv1 -> {
        1.0
      } +. {
        0.25
      }
    }
    "constructor" as whole, 7 if whole == "res" -> 0.1
    "data", 5 -> 0.5
    v12, _ -> case {
        let y = [10, 7]
        []
      }, Cv1 {
      [0], Cv1 -> 3.14
      [b, _, ..], Cv1 if b == 4 -> fn(v13) { v13 }(0.0)
      [], Cv1 -> 0.1
      v14, v15 -> 0.0
    }
  }
  echo 3
  echo seed_value
  echo False
}
