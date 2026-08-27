pub type Object {
  Cv0(value: String, inner: List(Int))
  Cv1
  Cv2(List(Int))
}

fn arguments(v3: String, y: List(Int)) -> Bool {
False
}

fn export(v4: List(Int)) -> Bool {
{
    case "b", "res" {
      "b" as whole, _ if whole != "x" -> fn(v5) { whole }(False)
      "data", "constructor" -> fn(v6) { "x" }(True)
      v7, _ -> "ab"
    }
  } |> arguments([4])
}

pub fn main() {
  let s = case Cv1, 3 {
    Cv2([5, ..rest]), v8 if v8 > 8 && v8 > 6 -> 0 - 1
    Cv0(_, []), _ -> {
      let s = 0
      s
    }
    Cv1, _ -> 42
    _, v9 -> 42 - v9
  }
  echo True
  echo case 42 {
    inner -> {
      fn(v10, v11) { v11 }(True, 10.0)
    } -. {
      {
        1.0
      } +. {
        0.25
      }
    }
    _ -> case 1.5, <<"data":utf8, "bc":utf8>> {
      3.14 as whole, <<"bc":utf8, "a":utf8, _:utf8>> -> {
        0.5
      } -. {
        1.0
      }
      0.1 as whole, <<_:utf8>> -> {
        let whole = "bc"
        1.5
      }
      _, _ -> {
        100.0
      } +. {
        2.0
      }
    }
  }
  echo False
  echo case 10.0 {
    v12 -> s
    _ -> case "x" == "constructor" {
      a -> s + s
      item -> {
        let s = [100, 10]
        42
      }
    }
    1.5 | 100.0 -> s
  }
}
