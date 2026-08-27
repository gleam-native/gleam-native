pub const golden_value: Bool = True
pub const euler_value: Float = 0.25
pub const pi_value: Bool = False

pub type Object {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn class(v0: Int, item: Bool, v1: Bool) -> String {
{
    {
      "b" <> "x"
    } <> "bc"
  } <> {
    case 0.1, "constructor" {
      v2, "abc" <> rest -> rest <> rest
      10.0 as whole, "" <> _ -> "x" <> "bc"
      0.5, "res" -> {
        let v1 = v1
        "data"
      }
      _, _ -> "ab"
    }
  }
}

fn f1(self_: List(Int), v3: Bool, v4: Bool) -> String {
class(2, v3, case "res" {
    "b" -> False
    "" <> rest -> v4
    _ -> v4
  })
}

fn f2(v5: String, v6: Int) -> String {
v5
}

pub fn main() {
  let s = "b"
  echo case fn(v7) { [3] }(False), {
      let new = [2]
      Record
    } {
    [], _ -> {
      fn(v8, v9) { euler_value }(0.1, False)
    } +. {
      3.14
    }
    [_, ..rest], v10 -> 1.5
    [b, constructor, ..], Record -> case Record {
      Record | Record -> euler_value +. {
        10.0
      }
      inner -> {
        let pair = []
        let l = False
        0.0
      }
      Record -> 1.5
    }
    v11, _ -> {
      let rest = golden_value
      let pair = 1.5
      0.1
    }
  }
  echo f2(case "a", Record {
    "bc" <> rest as whole, _ if whole != "data" || rest != "res" -> s
    "b", Record -> 4 |> class(True, golden_value)
    _, _ -> "ab"
  }, 100)
}
