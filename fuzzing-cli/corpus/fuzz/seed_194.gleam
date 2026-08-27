pub const euler_value: Float = 0.5
pub const seed_value: Bool = True
pub const limit_value: Int = 5

pub type Record {
  Cv0(value: String, inner: Float)
  None
}

fn f0(new: Record, l: Float, arguments: Record) -> List(Int) {
case "" <> "constructor" {
    "ab" <> rest | "ab" <> rest -> case #("bc", 3.14) {
      a -> fn(v1) { [] }(100)
      b -> [7, 100]
      a -> []
    }
    l -> [42, 4]
    "constructor" | "data" -> fn(v2, v3) { [] }(5, False)
  }
}

fn yield(v4: String, prototype: Int) -> Float {
10.0
}

fn new(v5: Record) -> Float {
case Cv0("a", 0.5) {
    None -> case "ab" <> "a", #(10, "bc") {
      "ab" <> rest, #(v5, "constructor") -> {
        0.25
      } *. {
        0.5
      }
      "data" as whole, #(1 as it, "a" <> rest) -> yield(whole, 5)
      v6, _ -> yield(v6, 3)
    }
    Cv0(item, _) -> 100.0
  }
}

pub fn main() {
  let rest = "bc"
  echo True
  echo case <<"bc":utf8, "x":utf8>> {
    <<4:1>> -> {
      euler_value <. {
        0.25
      }
    } && True
    _ -> seed_value
  }
  echo limit_value
  echo {
    let l = case [1, 2] {
      [2, 2, ..] -> f0(None, euler_value, Cv0("b", 1.0))
      [_, ..rest] -> None |> f0("b" |> yield(limit_value * 4), {
        let length = rest
        Cv0("res", 3.14)
      })
      [0] -> {
        let x = False
        [5, 0]
      }
      _ -> [5, 42]
    }
    let seed_value = case seed_value {
      _ -> True
      _ | True -> {
        2.0
      } <=. euler_value
    }
    case Cv0("bc", 3.14) {
      Cv0(inner, _) if inner != "bc" -> fn(v7) { [10] }(False)
      Cv0("data" <> rest, 3.14) -> None |> f0({
        let prototype = True
        let l = True
        euler_value
      }, None)
      Cv0(a, _) -> {
        let l = limit_value
        [4, 3]
      }
      v8 -> []
    }
  }
}
