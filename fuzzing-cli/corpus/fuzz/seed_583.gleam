pub type Object {
  Record
  Cv0
  Cv1(value: Float, inner: Float)
}

fn f0(v2: Object, v3: Int, v4: Int) -> Bool {
False
}

fn static(v5: Int) -> Float {
case False, Cv1(1.5, 0.1) {
    _, Cv1(_, _) -> 0.0
    False as whole, Record -> 0.1
    _, Record -> 1.5
    _, v6 -> {
      {
        10.0
      } +. {
        0.5
      }
    } -. {
      fn(v7, v8) { 10.0 }("res", 3)
    }
  }
}

fn f2(v9: Float, v10: String, prototype: Int) -> Bool {
{
    {
      prototype - prototype
    } % 2
  } != prototype
}

pub fn main() {
  let default = "b"
  let v = {
    let default = 2
    [42]
  }
  echo case "a" {
    a -> default
    "x" -> case {
        let constructor = v
        let constructor = v
        "bc"
      }, 0.0 {
      "abc" <> _, 0.5 as whole -> default <> "ab"
      _, v -> default <> default
    }
    b | "bc" <> b -> {
      default <> "abc"
    } <> "data"
  }
  echo case "" <> default, fn(v11, v12) { #(100, []) }(0.1, "b") {
    "x", #(9, [constructor, 5, ..]) -> case default <> "", 3.14 {
      "data" <> _, 10.0 -> {
        let this_ = [0]
        default
      }
      "constructor" <> rest, 2.0 -> "ab"
      _, _ -> "ab"
    }
    "res" <> _, #(_, []) -> default <> ""
    _, _ -> default
  }
  echo {
    2.0
  } -. {
    {
      let value = [2, 0]
      0.0
    }
  }
}
