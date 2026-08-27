pub type V0 {
  Cv1
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(n: String, v4: Int, v5: Int) -> Float {
case <<"x":utf8, 7:4>> {
    <<_:16, 10:8, "abc":utf8>> -> 1.0
    _ -> case <<2:8>>, Cv3 {
      <<_:8, "constructor":utf8>>, v6 -> 1.0
      _, _ -> fn(v7) { 1.0 }("x")
    }
  }
}

pub fn main() {
  echo case <<"x":utf8>> {
    <<"x":utf8>> -> fn(v8) { [] }("")
    _ -> [1, 3]
  }
  echo {
    {
      {
        let default = [2, 3]
        100
      }
    } - walk([4], 4)
  } < {
    0 - 5
  }
  echo {
    case "abc" {
      a -> 3.14
      "abc" <> rest if rest == "x" || rest == "ab" -> 3.14
      "data" -> {
        let length = [10]
        0.5
      }
    }
  } == {
    {
      0.5
    } +. {
      0.25
    }
  }
}
