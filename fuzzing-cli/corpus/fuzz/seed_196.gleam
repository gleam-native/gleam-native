pub const seed_value: Float = 3.14
pub const euler_value: Bool = False
pub const tag_value: Bool = True

fn constructor(constructor: Bool, rest: Bool, value: Int) -> Float {
{
    {
      {
        0.1
      } +. {
        10.0
      }
    } -. {
      1.5
    }
  } -. {
    2.0
  }
}

fn f1(value: Int, v0: Int, v1: String) -> Float {
{
    case #(100, "b") {
      constructor -> False
      #(4, "ab" <> _) -> fn(v2, v3) { v3 }("data", False)
      #(8, "a" <> _) | #(0, "res") -> fn(v4, v5) { True }(0.1, "constructor")
    }
  } |> constructor(!True, {
    let value = True
    let y = 0.1
    v0
  })
}

pub fn main() {
  let item = [1]
  let euler_value = item
  echo "res"
  echo case 0, 1 {
    9, item -> case "ab" <> "abc" {
      "x" | "a" -> {
        let new = tag_value
        let this_ = 2.0
        new
      }
      "b" <> rest | "a" <> rest -> tag_value
      "res" -> !False
      v6 -> tag_value
    }
    2, _ -> tag_value
    0, 8 -> case {
        let this_ = 0.1
        let n = [7, 2]
        "x"
      } {
      item -> fn(v7, v8) { tag_value }(0.1, 100.0)
      "data" -> tag_value
      item -> False
    }
    _, v9 -> False
  }
  echo {
    {
      1.5
    } *. {
      fn(v10, v11) { 1.0 }(100, 10)
    }
  } -. {
    {
      {
        let length = seed_value
        let seed_value = item
        100
      }
    } |> f1(2, {
      let pair = "ab"
      pair
    })
  }
}
