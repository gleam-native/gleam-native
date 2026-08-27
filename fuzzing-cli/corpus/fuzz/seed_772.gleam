pub const golden_value: String = "res"
pub const pi_value: Int = 42

pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

pub type Promise {
  Cv3(value: Float)
  Cv4(Int)
  Cv5
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(pair: Int, delete: Float) -> Int {
pair
}

fn f1(v6: Float, item: Int) -> Bool {
True
}

pub fn main() {
  let this_ = False
  echo case this_, 100 + pi_value {
    True, 9 as whole -> {
      let default = {
        0.0
      } -. {
        0.5
      }
      this_
    }
    True, 5 -> True
    _, v7 -> {
      {
        let length = [42]
        let l = this_
        "ab"
      }
    } != {
      {
        let l = 1.0
        golden_value
      }
    }
  }
  echo []
  echo case <<5:1, "":utf8>> {
    <<_:big-unsigned-8, _:utf8>> -> {
      fn(v8, v9) { "res" }("a", "res")
    } == "ab"
    _ -> case "a" <> golden_value, {
        let new = 42
        let arguments = new
        []
      } {
      _, [5, 5, ..] -> {
        0.0
      } <=. {
        0.1
      }
      _, [3, 5, ..] -> 4 != pi_value
      "ab" as whole, [h, ..rest] -> this_
      _, v10 -> "data" == golden_value
    }
  }
}
