pub const seed_value: Float = 0.5
pub const pi_value: Float = 0.25
pub const tag_value: Bool = True

pub type V0 {
  Ok(value: String, inner: Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn default(self_: Bool) -> String {
{
    case "" <> "b" {
      "a" as whole if whole == "ab" || whole != "a" -> whole
      inner | "bc" <> inner -> fn(v1, v2) { "constructor" }(True, 3)
      _ -> "abc"
    }
  } <> {
    {
      let acc = 0 * 3
      let acc = "constructor"
      "b"
    }
  }
}

pub fn main() {
  let l = 1
  let this_ = case <<100:4, 0:4>>, True {
    <<_:utf8, "constructor":utf8>>, _ -> []
    _, _ -> []
  }
  echo case this_ {
    [a] -> seed_value == {
      {
        1.0
      } -. seed_value
    }
    [_, ..rest] -> {
      {
        let v = tag_value
        False
      }
    } || True
    [2] -> {
      pi_value *. seed_value
    } == pi_value
    v3 -> case l % 7 {
      item -> False
      a -> True
    }
  }
  echo fn(v4, v5) { case v4, this_ {
    4, [3] -> seed_value
    6, [0, ..rest] as whole -> pi_value
    3, [] -> 100.0
    _, v6 -> fn(v7) { 1.5 }(4)
  } }(5, True)
  echo {
    case Ok("ab", 100.0) {
      Ok(_, inner) -> "bc" <> "res"
      Ok(_, 0.0) -> "data"
      Ok("data" as whole, 100.0) -> default(True)
    }
  } <> "a"
  echo case l, {
      let z = True
      let default = []
      True
    } {
    6, True -> {
      10.0
    } /. {
      3.14
    }
    8, False -> pi_value
    _, _ -> pi_value +. {
      fn(v8) { v8 }(2.0)
    }
  }
}
