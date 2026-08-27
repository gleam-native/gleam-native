pub const seed_value: Bool = False

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  None(Int)
}

pub type V2 {
  Some(value: Int, inner: String)
  Cv3
}

fn f0(arguments: List(Int), v4: #(Bool, String), v: Int) -> String {
{
    case Cv1([2, 2], 3), {
        let self_ = v
        "data"
      } {
      _, "res" -> "data"
      None(3), "b" <> rest -> ""
      v5, v6 -> "abc" <> "b"
    }
  } <> {
    fn(v7, v8) { {
      let m = arguments
      "x"
    } }(3.14, 2.0)
  }
}

fn f1(l: V2, new: V0, x: Float) -> String {
[] |> f0(#(False, "abc"), 3 + 10)
}

fn static(v9: V0, prototype: Bool, m: String) -> Int {
case fn(v10) { m }(3) {
    "bc" | "data" -> case 2 {
      7 -> {
        let default = prototype
        let rest = m
        2
      }
      inner -> {
        let self_ = [10]
        let v9 = m
        inner
      }
      b -> 100
    }
    "x" -> 4
    _ -> {
      let default = [7]
      let rest = 2
      fn(v11) { 5 }(2)
    }
  }
}

pub fn main() {
  let arguments = {
    let x = {
      0.5
    } +. {
      2.0
    }
    let y = fn(v12) { 10 }(1.0)
    {
      let self_ = [2]
      let arguments = 100
      [42]
    }
  }
  echo fn(v13, v14) { {
    let seed_value = 0 + 2
    {
      let arguments = "a"
      let pair = arguments
      False
    }
  } }("", "x")
  echo 0.1
}
