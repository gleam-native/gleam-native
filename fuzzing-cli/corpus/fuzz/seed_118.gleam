pub type V0 {
  Record(value: String, inner: String)
  Some(value: Bool)
  Cv1(value: Int)
}

pub type V2 {
  Ok(value: Bool, inner: Float)
}

pub type V3 {
  Cv4(value: Int)
  Cv5
  Cv6(value: Int, inner: Int)
}

fn f0(rest: String) -> Float {
case {
      let rest = 3
      Ok(False, 3.14)
    } {
    Ok(b, _) -> case "ab" <> rest {
      "a" <> a -> 0.5
      a -> 10.0
      constructor -> {
        0.1
      } -. {
        10.0
      }
    }
    constructor -> {
      0.5
    } /. {
      0.5
    }
  }
}

fn f1(v7: V0, z: Bool, arguments: List(Int)) -> Bool {
fn(v8, v9) { "a" == {
    fn(v10, v11) { "b" }(0.1, 7)
  } }(1.0, False)
}

fn f2(class: #(Int, List(Int)), prototype: List(Int), v12: List(Int)) -> Int {
{
    4 + {
      42 + 5
    }
  } + {
    case {
        let class = False
        let v12 = "ab"
        10
      }, "ab" <> "res" {
      2, _ -> 4 + 7
      _, "b" <> rest -> 10 - 4
      _, v13 -> 100 * 2
    }
  }
}

pub fn main() {
  let value = 100.0
  let m = case {
      10.0
    } /. {
      1.0
    }, f0("x") {
    10.0, 3.14 as whole -> 3
    arguments, value -> 0 % 5
    v14, 3.14 as whole -> 1
  }
  echo fn(v15) { case "bc" {
    "a" <> item if item != "a" && item != "bc" -> ""
    "x" as whole -> "bc"
    _ -> "constructor"
  } }(1.5)
}
