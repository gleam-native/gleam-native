pub const euler_value: String = "res"
pub const tag_value: Bool = False
pub const golden_value: Float = 0.1

pub type Record {
  Cv0(value: String, inner: List(Int))
  None
}

pub type V1 {
  Number(value: Int, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(item: Int, this_: Int, v2: Bool) -> Int {
case Cv0("", [0, 100]) {
    item -> this_
    None as whole -> walk([], 42)
    _ -> item
  }
}

fn f1(v3: #(Bool, String)) -> Bool {
{
    case fn(v4) { Number(5, 0.1) }(2.0), 2 + 1 {
      v5, 6 -> 0.5
      Number(n, v6), 7 -> v6
      Number(v7, 0.0), _ -> {
        1.5
      } +. {
        1.5
      }
      _, _ -> fn(v8, v9) { 1.0 }(True, 3.14)
    }
  } != {
    case "bc" <> "bc" {
      constructor | "data" <> constructor -> 0.25
      "a" <> rest as whole -> {
        0.25
      } -. {
        0.25
      }
    }
  }
}

fn f2(s: List(Int), v10: List(Int), v11: #(Float, Bool)) -> List(Int) {
s
}

pub fn main() {
  echo 0
  echo {
    {
      let golden_value = "x"
      let n = False
      {
        let default = 2.0
        []
      }
    }
  } |> f2(f2([42], [100, 4], #(0.25, True)), #(0.5, False))
}
