pub const golden_value: Float = 0.1
pub const seed_value: Float = 2.0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(value: Int)
  Cv3(String)
}

pub type Symbol {
  Record
}

fn default(value: Float, item: Float, self_: #(Int, Bool)) -> Bool {
{
    let arguments = {
      fn(v4, v5) { v4 }(7, 100)
    } - 3
    let arguments = {
      {
        10.0
      } -. value
    } >=. value
    arguments
  }
}

pub fn main() {
  let l = True || False
  let golden_value = 3
  echo {
    {
      let rest = "data"
      let acc = 0.25
      {
        let new = l
        let acc = False
        "b"
      }
    }
  } <> {
    case fn(v6, v7) { golden_value }(False, 1.5), golden_value + golden_value {
      4, 9 -> "ab" <> "data"
      1, 6 as whole -> "data"
      3, _ -> "b"
      _, v8 -> "constructor"
    }
  }
  echo fn(v9, v10) { "bc" }(2.0, "a")
  echo 0
  echo case #([], "bc") {
    #([8, ..rest], _) -> {
      "ab" <> "ab"
    } <> "abc"
    inner -> "res"
    #([4, ..rest], "ab") -> "a" <> {
      "res" <> "ab"
    }
  }
}
