pub const tag_value: Bool = False
pub const seed_value: Float = 2.0
pub const limit_value: Float = 2.0

pub type Number {
  Cv0(value: String, inner: Bool)
  Cv1(Bool)
  Cv2(String, Float)
}

pub type Map {
  Error(Bool)
}

pub type V3 {
  Cv4(value: String, inner: List(Int))
}

fn f0(v5: List(Int), v6: String, constructor: #(Bool, List(Int))) -> Bool {
True
}

fn new(delete: String) -> Float {
1.0
}

fn f2(z: Float, class: Bool, this_: Int) -> Int {
case [1, 5] {
    [8, 5, ..] -> 7 * {
      this_ * this_
    }
    [6, ..rest] -> {
      fn(v7, v8) { this_ }(2.0, 10)
    } - {
      fn(v9) { this_ }(0.0)
    }
    [_, 4, ..] -> this_ - 100
    v10 -> {
      2 + 4
    } + {
      this_ + this_
    }
  }
}

pub fn main() {
  let tag_value = [2]
  echo "ab"
  echo False
  echo {
    fn(v11) { seed_value }(0.25)
  } -. {
    {
      {
        let length = seed_value
        let item = ""
        length
      }
    } +. {
      "constructor" |> new()
    }
  }
  echo {
    let delete = case fn(v12) { Cv4("x", [2]) }(False), #([7, 10], [42, 100]) {
      Cv4("res" <> _, []), #([5], [_, 5, ..]) -> True
      Cv4("abc" <> _, [2]), #([seed_value, ..rest] as whole, [] as it) -> True && False
      _, v13 -> {
        let s = True
        s
      }
    }
    case delete {
      _ | True -> tag_value |> f0("data" <> "a", #(True, [5]))
      v14 -> fn(v15) { True }(3.14)
    }
  }
}
