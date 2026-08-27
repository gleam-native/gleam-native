pub const seed_value: String = "ab"

pub type V0 {
  Record(value: String, inner: List(Int))
  Cv1(value: List(Int))
  Cv2
}

pub type Symbol {
  Error
  Cv3
  Cv4
}

pub type Map {
  Cv5(List(Int))
  Number
}

fn f0(this_: Bool) -> Int {
4 - {
    {
      let self_ = {
        0.25
      } +. {
        3.14
      }
      let constructor = 5
      constructor
    }
  }
}

fn f1(v6: String, v: Float, pair: Bool) -> List(Int) {
fn(v7, v8) { case v8 <> v6 {
    item -> []
    "a" <> rest -> [100, 4]
  } }(0.0, "abc")
}

fn extends(v9: Int, v10: Int, v11: Bool) -> Float {
{
    case {
        let v10 = v10
        let v10 = 2.0
        "b"
      } {
      "ab" <> rest -> fn(v12) { 10.0 }(2)
      "res" -> {
        2.0
      } +. {
        2.0
      }
      _ -> fn(v13, v14) { v14 }(True, 0.0)
    }
  } *. {
    {
      {
        let constructor = 3
        1.5
      }
    } -. {
      10.0
    }
  }
}

pub fn main() {
  let default = {
    let x = [100, 3]
    let x = [7, 3]
    seed_value <> "bc"
  }
  echo case {
      let seed_value = 10
      let seed_value = "a"
      True
    } {
    v15 -> "a" <> {
      fn(v16) { seed_value }(0)
    }
    inner -> case Cv4 {
      Cv3 -> ""
      inner -> "abc"
      item -> "a" <> default
    }
  }
  echo {
    extends(1, 1, False) /. {
      0.5
    }
  } -. {
    1.0
  }
}
