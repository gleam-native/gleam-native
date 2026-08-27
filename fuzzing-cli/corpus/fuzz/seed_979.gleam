pub const golden_value: Float = 0.5

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Bool)
  Cv3(value: List(Int))
}

pub type Promise {
  Cv4(value: Int)
  Cv5
}

pub type V6 {
  Ok
  Cv7(Bool)
  Number(String)
}

fn f0(v8: #(Bool, Float)) -> List(Int) {
case [10, 10] {
    [h, ..rest] -> fn(v9, v10) { [42, 42] }(1, "res")
    [v8] -> case v8 * v8, "constructor" {
      0, "bc" <> _ -> [1]
      8 as whole, "b" if whole > 6 && whole <= 1 -> fn(v11, v12) { [100, 7] }(4, "bc")
      5, "constructor" <> _ -> [10, 3]
      _, _ -> []
    }
    [_, 0, ..] -> []
    _ -> [0, 4]
  }
}

fn export(v13: Int) -> Float {
{
    0.5
  } /. {
    1.0
  }
}

pub fn main() {
  let golden_value = {
    {
      100.0
    } +. golden_value
  } -. export(0)
  let s = golden_value == {
    {
      1.0
    } /. {
      3.14
    }
  }
  echo {
    let v = case {
        let constructor = golden_value
        #([1, 4], 0)
      } {
      #([4, ..rest], s) -> {
        let v = []
        let pair = [3, 4]
        "x"
      }
      #([x, ..rest], 8) -> "data"
      #([9, ..rest] as whole, 9) -> "bc"
      _ -> "x" <> "bc"
    }
    let v = 7
    {
      {
        let constructor = 7
        let y = "b"
        golden_value
      }
    } -. {
      fn(v14, v15) { 2.0 }(False, "data")
    }
  }
  echo False
  echo {
    let n = f0(#(True, 0.0))
    {
      100 + 1
    } - 1
  }
}
