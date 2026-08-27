pub const pi_value: Bool = False

pub type Map {
  Record
  Cv0(Bool)
  Ok
}

pub type Promise {
  Cv1(Bool, value: Bool)
  Cv2(value: Bool)
}

pub type V3 {
  Some
}

fn f0(v4: String, v5: Bool, y: V3) -> Float {
{
    {
      let class = 1
      let class = {
        let class = True
        let this_ = v4
        [100, 42]
      }
      {
        100.0
      } -. {
        1.5
      }
    }
  } +. {
    case fn(v6) { 100 }(0.5) {
      item -> {
        let n = 1.5
        let n = n
        10.0
      }
      b -> {
        10.0
      } *. {
        3.14
      }
      constructor -> {
        let class = 10.0
        let prototype = v4
        100.0
      }
    }
  }
}

pub fn main() {
  let y = {
    fn(v7, v8) { 100 }(True, 10)
  } == {
    5 - 1
  }
  echo [2, 7]
  echo case {
      let constructor = "a"
      Cv2(False)
    } {
    item -> y
    Cv1(True, False) -> case {
        let pi_value = 0.5
        let y = y
        "res"
      }, fn(v9) { True }("abc") {
      "constructor", _ -> {
        let class = [10]
        let default = []
        True
      }
      "ab", pi_value if pi_value -> pi_value
      _, _ -> {
        0.1
      } != {
        2.0
      }
    }
    Cv1(True, length) -> {
      3 * 5
    } != 2
  }
  echo case <<"bc":utf8, 2:1>> {
    <<_:little-signed-8, _:utf8>> -> case "" {
      "data" <> _ | "res" <> _ -> {
        let y = False
        let y = 10
        "ab"
      }
      "a" -> {
        let pi_value = pi_value
        let pi_value = False
        "bc"
      }
      _ -> "x" <> "b"
    }
    <<_:1, _:utf8>> -> {
      "bc" <> "x"
    } <> {
      "x" <> "bc"
    }
    _ -> {
      "a" <> "a"
    } <> "a"
  }
  echo case Some {
    inner -> "data"
    Some | Some -> "bc"
    _ | Some -> case 0 + 42 {
      _ -> "a" <> "b"
      inner -> "res" <> "ab"
    }
  }
}
