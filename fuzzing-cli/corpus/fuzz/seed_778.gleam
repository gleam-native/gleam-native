pub type V0 {
  Cv1(value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(v: Bool, v2: #(Bool, Bool), v3: V0) -> String {
fn(v4) { "x" }("ab")
}

fn extends(y: String, l: Bool) -> Int {
3
}

pub fn main() {
  echo case <<3:16, "res":utf8>> {
    <<"res":utf8, "a":utf8, _:4>> -> case #(100.0, 3) {
      #(2.0, 6) -> True
      b -> True
      #(10.0, _) as whole -> fn(v5) { True }("a")
    }
    <<42:1, "a":utf8>> -> case fn(v6, v7) { v6 }(2, False) {
      8 as whole if whole <= 6 -> "ab" == "res"
      7 -> False
      6 -> 3 != 5
      v8 -> "bc" == "a"
    }
    _ -> {
      {
        0.25
      } -. {
        2.0
      }
    } == {
      {
        100.0
      } +. {
        1.0
      }
    }
  }
  echo {
    1.0
  } +. {
    case 0.1 {
      3.14 -> 100.0
      3.14 -> {
        2.0
      } +. {
        2.0
      }
      v9 -> 10.0
    }
  }
  echo 0 <= {
    {
      fn(v10, v11) { v10 }("ab", 3)
    } |> extends(True || True)
  }
  echo {
    case fn(v12) { Cv1([]) }(3.14), "res" <> "" {
      _, _ -> {
        let default = []
        []
      }
      Cv1([2, ..rest]), "x" -> []
    }
  } |> walk(7)
}
