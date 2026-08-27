pub const limit_value: Float = 0.0

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Cv3(List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(default: Int) -> String {
{
    let x = case #(False, 1), Cv1([100, 10]) {
      #(False, this_), Cv1([4, _, ..]) if this_ > 9 -> 2.0
      #(False, default), v4 -> {
        3.14
      } /. {
        2.0
      }
      _, v5 -> {
        1.5
      } *. {
        1.5
      }
    }
    let m = fn(v6, v7) { "x" }(False, 10.0)
    "b"
  }
}

fn f1(class: Float, v8: List(Int)) -> Bool {
{
    let new = case {
        let v8 = True
        #(False, "bc")
      } {
      constructor -> v8
      #(True, "ab") | #(True, "" <> _) -> fn(v9, v10) { v8 }(3.14, True)
    }
    {
      1 % 3
    } != spin(0, 0)
  }
}

fn f2(n: Int, v11: #(Int, Bool), v12: String) -> Float {
1.0
}

pub fn main() {
  let class = 1
  let s = False
  echo case limit_value -. {
      1.5
    } {
    a -> class
    class -> 0 - 10
    2.0 | 1.5 -> {
      let delete = limit_value == limit_value
      let v = f0(4)
      class
    }
  }
  echo {
    2.0
  } *. {
    {
      class |> f2(#(2, True), "")
    } /. {
      1.0
    }
  }
  echo {
    {
      fn(v13, v14) { "" }(False, False)
    } <> {
      "abc" <> "res"
    }
  } != {
    fn(v15, v16) { "ab" }(10.0, 0.1)
  }
}
