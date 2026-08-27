pub type V0 {
  None(value: String, inner: String)
}

pub type Map {
  Error
}

pub type V1 {
  Cv2
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(arguments: Float) -> List(Int) {
case {
      let class = False
      let class = 42
      Cv2
    } {
    a -> fn(v4) { [5] }(3)
    constructor -> case 42, [100, 42] {
      2, [constructor] if constructor > 4 -> [1]
      1, [9] as whole -> []
      v5, v6 -> {
        let new = "constructor"
        let item = True
        [7, 42]
      }
    }
  }
}

pub fn main() {
  let s = 2.0
  let s = {
    1.0
  } -. {
    {
      1.5
    } +. s
  }
  echo case [3], <<"a":utf8>> {
    [], <<_:utf8>> -> case fn(v7, v8) { "constructor" }(False, True) {
      "a" -> 0.1
      inner -> {
        0.1
      } -. s
      "res" as whole -> 10.0
    }
    [4], <<_:utf8, "bc":utf8>> -> case 1.5, {
        let item = 100
        Cv3
      } {
      1.5, _ -> s -. s
      1.5, Cv3 -> 0.5
      100.0, Cv2 -> {
        0.0
      } -. s
      _, _ -> s
    }
    [6, h, ..], _ -> fn(v9, v10) { v9 }(1.0, 0.25)
    _, v11 -> s
  }
  echo case {
      let v = 4
      let this_ = [0]
      #([7, 3], [])
    } {
    #([6] as whole, [4]) -> {
      {
        0.0
      } +. s
    } |> class()
    #([], []) -> fn(v12, v13) { [] }(True, 0.1)
    #([_, ..rest], [5]) -> fn(v14) { {
      let y = s
      rest
    } }(True)
    v15 -> [5]
  }
  echo case 5 % 1 {
    a -> case None("", "x") {
      None("x" <> rest, "" <> _) -> fn(v16, v17) { True }(1, "bc")
      item -> fn(v18, v19) { False }("abc", False)
      constructor -> {
        let arguments = True
        arguments
      }
    }
    5 -> False
    _ -> case <<"bc":utf8, "bc":utf8>> {
      <<_:16>> -> fn(v20, v21) { False }(True, False)
      <<_:utf8>> -> fn(v22) { True }(1.0)
      <<_:16, 5:8, "abc":utf8>> -> fn(v23, v24) { v24 }(10.0, True)
      _ -> 7 <= 1
    }
  }
  echo case "a" != "a", {
      let delete = [0, 2]
      0
    } {
    False, 3 -> case "data" <> "ab", 0 {
      "x" as whole, 6 -> walk([1], 3)
      s, _ -> {
        let arguments = 42
        let arguments = "ab"
        100
      }
    }
    _, 1 as whole if whole == 4 || whole > 5 -> 5 - {
      whole - 2
    }
    False, 2 -> [2, 1] |> walk(fn(v25) { 3 }(10.0))
    _, _ -> {
      let default = 1.5
      let pair = {
        let s = False
        default
      }
      7
    }
  }
}
