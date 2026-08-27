pub const seed_value: Bool = False

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: List(Int), v0: Float, x: Bool) -> Float {
1.5
}

pub fn main() {
  echo case "x" <> "x" {
    "x" <> _ | "data" <> _ -> False
    item -> False
    "bc" -> seed_value
  }
  echo "bc"
  echo case <<10:1, 42:16, "a":utf8>> {
    <<_:1>> -> 1.5
    <<_:8>> -> case {
        let x = [0]
        let default = []
        42
      }, {
        let item = 0.0
        1
      } {
      v, _ -> f0([2, 100], 10.0, True)
      1, 0 -> {
        3.14
      } /. {
        10.0
      }
    }
    _ -> 0.5
  }
  echo {
    seed_value && {
      {
        1.5
      } >=. {
        1.5
      }
    }
  } || {
    {
      {
        let length = seed_value
        let length = "b"
        True
      }
    } && {
      True && False
    }
  }
}
