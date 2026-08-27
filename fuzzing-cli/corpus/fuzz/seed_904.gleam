pub const limit_value: Float = 100.0

pub type Number {
  Record
  Cv0(Float)
  None(value: Bool)
}

fn extends(v1: Int) -> Bool {
True
}

pub fn main() {
  echo {
    0 - {
      42 + 0
    }
  } - {
    0 - {
      2 % 6
    }
  }
  echo case limit_value {
    b -> {
      limit_value +. b
    } +. {
      {
        let acc = True
        limit_value
      }
    }
    1.0 -> {
      10.0
    } -. {
      {
        let limit_value = False
        let m = "bc"
        1.5
      }
    }
  }
  echo limit_value >=. {
    fn(v2) { {
      100.0
    } -. {
      100.0
    } }(0.25)
  }
  echo case {
      let z = False
      ""
    }, 2 {
    "res" as whole, 6 -> [1, 10]
    "b" <> _, pair -> {
      let pair = extends(pair)
      []
    }
    _, v3 -> []
  }
}
