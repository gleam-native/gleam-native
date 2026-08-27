pub const tag_value: Bool = False

pub type Symbol {
  Record
  Cv0(value: List(Int))
}

pub type Number {
  Cv1(value: Int)
  Cv2
}

fn yield(v: Float) -> List(Int) {
{
    let v = {
      let s = v -. v
      let v = ""
      s
    }
    case "a" <> "data" {
      item | "x" <> item -> [7]
      "a" <> inner -> fn(v3, v4) { [] }(True, False)
      v5 | "data" <> v5 -> fn(v6, v7) { [] }("x", False)
    }
  }
}

fn f1(prototype: String, length: Symbol, x: Bool) -> Int {
case 0 - 4, yield(0.25) {
    length, [b] if b <= 9 -> case prototype <> prototype, fn(v8) { Cv2 }(0.1) {
      "b" <> rest, Cv2 if rest != "abc" || rest == "abc" -> 7 + 7
      "a", Cv2 -> 2
      _, _ -> b + 42
    }
    2, [3] -> {
      0 + 4
    } + 1
    _, _ -> 7
  }
}

pub fn main() {
  let y = yield({
    0.0
  } *. {
    0.1
  })
  let item = 0.5
  echo item -. {
    {
      0.25
    } +. {
      item +. item
    }
  }
  echo [2, 10]
  echo fn(v9, v10) { case {
      let length = 1.5
      "constructor"
    } {
    _ -> v9
    inner -> 0.0
    item | "" <> item -> {
      100.0
    } -. {
      0.5
    }
  } }(2.0, False)
  echo case Cv2 {
    _ | Cv2 -> "constructor"
    b -> "a"
  }
}
