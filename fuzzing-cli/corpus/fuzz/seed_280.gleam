pub const golden_value: Bool = False
pub const seed_value: Int = 7

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(value: Bool)
  Cv4(Bool, value: String)
  Cv5(value: Float, inner: Int)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(v6: Float, v7: Int, v: Float) -> Int {
{
    0 - {
      3 - v7
    }
  } |> spin(2)
}

fn f1(v8: String, v9: Bool, v10: String) -> List(Int) {
[10, 5]
}

fn f2(prototype: Int, v11: Int) -> List(Int) {
{
    let l = case v11 {
      5 -> {
        let this_ = "ab"
        [10]
      }
      inner -> {
        let inner = []
        let v11 = "constructor"
        [10]
      }
    }
    let n = {
      let l = [2]
      let value = {
        0.5
      } /. {
        0.5
      }
      value == value
    }
    {
      let value = "ab" |> f1(n, "" <> "constructor")
      [7]
    }
  }
}

pub fn main() {
  let pair = "bc"
  echo f0(fn(v12, v13) { v12 -. v12 }(10.0, 3.14), 42 - seed_value, {
    let m = pair
    {
      1.5
    } -. {
      2.0
    }
  })
  echo case [] {
    [] -> 3.14
    [] -> 0.5
    _ -> fn(v14) { 0.5 }(5)
  }
}
