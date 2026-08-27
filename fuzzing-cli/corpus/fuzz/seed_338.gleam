pub const euler_value: Float = 2.0
pub const limit_value: Bool = True
pub const golden_value: Float = 0.25

pub type Record {
  Cv0(value: String, inner: Float)
  Cv1(String, String)
  Some(String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v: Float, v2: #(Bool, List(Int)), prototype: Bool) -> Float {
{
    {
      2.0
    } +. {
      {
        0.25
      } -. v
    }
  } *. {
    case "data", {
        let v2 = "abc"
        let acc = prototype
        Cv0("a", 0.1)
      } {
      v, Some("res") -> 10.0
      "a", _ -> 10.0
      _, _ -> {
        2.0
      } -. {
        100.0
      }
    }
  }
}

fn f1(class: Int, v3: Int) -> String {
case 0.5 {
    a -> "b"
    0.5 -> case "a" <> "", "abc" <> "x" {
      "bc", "ab" -> "x" <> "a"
      v, _ -> v
      "b" <> rest, _ -> {
        let this_ = rest
        "data"
      }
    }
    1.5 | 1.5 -> "a"
  }
}

pub fn main() {
  echo [7]
  echo "res" <> {
    {
      fn(v4, v5) { "a" }(0.0, 1)
    } <> "a"
  }
  echo {
    let limit_value = case fn(v6) { v6 }(2), 2 |> f1(fn(v7) { 100 }(True)) {
      v8, _ -> {
        let item = []
        v8
      }
      _, "b" <> _ -> 100 - 42
    }
    case "" {
      "x" <> constructor | "b" <> constructor -> constructor <> constructor
      "ab" as whole -> whole <> whole
      v9 -> v9
    }
  }
}
