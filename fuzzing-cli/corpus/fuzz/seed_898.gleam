pub const euler_value: Int = 2

pub type V0 {
  Error(value: String, inner: String)
  Some(String)
  Cv1(value: Int, inner: Int)
}

pub type V2 {
  Cv3(List(Int), value: Int)
  Cv4
  Cv5(value: Bool)
}

fn extends(this_: #(Int, String)) -> List(Int) {
[10, 5]
}

fn f1(v6: V2, v7: String, n: Int) -> Bool {
case fn(v8, v9) { v8 }(2.0, True) {
    v7 -> case {
        let class = [10, 5]
        n
      } {
      3 -> {
        let y = n
        True
      }
      b -> fn(v10) { False }(42)
    }
    100.0 -> True
    _ -> True
  }
}

pub fn main() {
  let euler_value = {
    100.0
  } *. {
    {
      1.0
    } -. {
      1.0
    }
  }
  let arguments = fn(v11, v12) { 3.14 }("a", "data")
  echo extends(#(4, "x"))
  echo case [10] {
    [4] -> euler_value
    [arguments, 3, ..] if arguments == 0 -> case <<"a":utf8>>, [] {
      <<"":utf8, 0:16>>, [arguments, 4, ..] if arguments > 9 -> fn(v13, v14) { 0.0 }("x", 2.0)
      _, [2] -> euler_value
      _, _ -> euler_value
    }
    [] -> {
      let acc = arguments
      let class = arguments +. euler_value
      {
        2.0
      } *. acc
    }
    _ -> {
      arguments -. {
        0.1
      }
    } +. {
      {
        let self_ = arguments
        1.5
      }
    }
  }
}
