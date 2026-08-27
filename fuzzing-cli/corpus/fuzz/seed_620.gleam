pub type Promise {
  Cv0(value: String, inner: List(Int))
}

pub type V1 {
  Cv2
  Cv3(value: List(Int), inner: List(Int))
}

pub type V4 {
  Cv5(value: Bool)
  Cv6
  Ok(String, Float)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(item: Bool, prototype: List(Int), class: String) -> Bool {
case class <> "b" {
    constructor -> {
      fn(v7) { 5 }(42)
    } >= 7
    inner -> {
      0.25
    } >=. {
      {
        1.0
      } *. {
        0.25
      }
    }
  }
}

fn f1(class: Int) -> Float {
{
    fn(v8) { v8 -. v8 }(0.0)
  } -. {
    case 2.0 {
      2.0 | 0.0 -> {
        100.0
      } +. {
        1.5
      }
      this_ -> {
        let x = class
        let x = []
        10.0
      }
      b -> 10.0
    }
  }
}

fn f2(v9: Float, v10: Int) -> String {
"ab"
}

pub fn main() {
  let new = False
  let prototype = case [], Cv0("data", [4, 100]) {
    [], _ -> [10, 5]
    [new], _ if new <= 3 -> {
      let new = [3]
      []
    }
    [7, ..rest], Cv0(_, []) -> rest
    v11, v12 -> v11
  }
  echo fn(v13) { case {
      100.0
    } +. {
      0.5
    }, <<"b":utf8>> {
    _, <<"data":utf8, 1:1, _:utf8>> -> 3.14
    10.0, <<_:utf8>> -> {
      1.0
    } -. {
      2.0
    }
    2.0, <<0:16, 1:8>> -> 1.0
    _, _ -> {
      let default = prototype
      let new = "data"
      0.25
    }
  } }(5)
}
