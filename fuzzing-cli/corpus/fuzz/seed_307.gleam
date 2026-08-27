pub const tag_value: String = ""
pub const euler_value: Int = 7

pub type Record {
  Cv0(value: String, inner: Float)
  Cv1(value: Int)
  Cv2(List(Int), Int)
}

pub type V3 {
  Cv4(List(Int), Float)
}

pub type V5 {
  Some
}

fn extends(pair: Int) -> Bool {
case "data" <> "b" {
    v6 | "data" <> v6 -> {
      0.5
    } <. {
      2.0
    }
    "a" -> True
  }
}

pub fn main() {
  echo case {
      let y = [42, 1]
      let y = False
      "res"
    }, {
      let default = [3, 4]
      "abc"
    } {
    "res" <> rest as whole, "abc" <> tail if rest != "bc" || whole == "abc" -> [100]
    "abc", "data" -> case Some, euler_value - 10 {
      _, 7 -> [42, 10]
      Some, 9 -> {
        let arguments = True
        [100]
      }
      Some, 8 -> [0, 0]
      v7, _ -> [0, 5]
    }
    "a" <> _, "bc" -> [100]
    v8, _ -> case fn(v9) { [] }(2) {
      [] -> []
      [a] -> [0]
      _ -> []
    }
  }
  echo [2, 0]
  echo {
    let item = False
    let euler_value = case [4], 10.0 {
      [item], 0.0 if item > 2 -> []
      [], _ -> []
      [5, 4, ..], 10.0 -> []
      _, _ -> [4, 100]
    }
    {
      let item = {
        0.25
      } -. {
        3.14
      }
      tag_value <> tag_value
    }
  }
}
