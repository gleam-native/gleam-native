pub const limit_value: Float = 3.14
pub const euler_value: Int = 42
pub const seed_value: Float = 10.0

pub type Record {
  Cv0(value: String, inner: String)
  Cv1(value: String)
}

fn constructor(v2: Bool) -> Bool {
False
}

fn f1(default: List(Int), new: List(Int), arguments: String) -> Float {
{
    0.5
  } +. {
    2.0
  }
}

pub fn main() {
  let delete = case [4, 2] {
    [9, ..rest] -> False
    [5, limit_value, ..] -> True
    _ -> True |> constructor()
  }
  echo case fn(v3) { Cv1("res") }(5) {
    b -> case {
        let m = [4, 2]
        let n = False
        "constructor"
      }, {
        let m = 4
        let euler_value = 0.5
        b
      } {
      "ab" as whole, Cv1("x" <> _) if whole != "data" -> 0.0
      b, Cv0("bc", "a") if b != "" || b != "" -> seed_value
      "x" as whole, Cv0("bc" <> _ as it, "b" as subject_) -> f1([42, 3], [10, 7], it)
      _, _ -> limit_value
    }
    inner -> f1([0], {
      let y = [42, 2]
      []
    }, "res" <> "bc")
    v4 -> case "b" <> "constructor" {
      delete | "b" <> delete -> seed_value -. {
        100.0
      }
      "res" | "ab" -> seed_value
      a | "" <> a -> f1([42, 2], [1], "ab")
    }
  }
  echo {
    case fn(v5) { v5 }(3) {
      _ | 5 -> seed_value +. {
        2.0
      }
      1 | 3 -> 0.5
      0 | 7 -> 3.14
    }
  } +. seed_value
  echo limit_value
  echo {
    let v = {
      "res" != "a"
    } && {
      delete && delete
    }
    let z = seed_value /. {
      10.0
    }
    case 2 {
      b -> b + euler_value
      a -> euler_value
    }
  }
}
