pub const golden_value: Bool = True
pub const seed_value: Int = 2

pub type V0 {
  Number(value: String, inner: List(Int))
  Cv1(Int, value: List(Int))
}

pub type Promise {
  None
  Record(value: Int, inner: String)
}

pub type V2 {
  Error
}

fn f0(this_: Int, constructor: Promise) -> Float {
0.1
}

fn f1(constructor: List(Int)) -> String {
""
}

fn f2(v3: List(Int)) -> Float {
0.0
}

pub fn main() {
  echo True
  echo {
    let pair = case "constructor", [] |> f1() {
      "abc" as whole, _ if whole == "abc" || whole == "b" -> False
      "res", _ -> 0 == seed_value
      _, _ -> golden_value
    }
    let this_ = {
      let default = 3
      let self_ = 10.0
      {
        1.0
      } != self_
    }
    {
      42 * seed_value
    } % 1
  }
  echo fn(v4) { False }(7)
}
