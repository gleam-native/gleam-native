pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(Bool)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn arguments(this_: Int, m: Int, prototype: Int) -> Int {
7
}

pub fn main() {
  let v = False
  let v = 0
  echo {
    let l = False
    let arguments = {
      v % 2
    } + {
      v + 4
    }
    1.0
  }
  echo case <<2:16, "ab":utf8, 0:16>>, <<"res":utf8, "data":utf8, 42:8>> {
    <<_:1>>, <<"data":utf8, length:8>> -> 2.0
    <<100:1, _:8, "":utf8>>, _ -> case arguments(4, 2, 10), 0 - 100 {
      3 as whole, 9 -> {
        0.1
      } -. {
        1.0
      }
      2, _ -> 0.1
      _, _ -> {
        let arguments = v
        0.5
      }
    }
    _, _ -> case v + v, {
        10.0
      } /. {
        3.14
      } {
      _, v4 -> v4 -. {
        100.0
      }
      3, _ -> {
        0.1
      } *. {
        100.0
      }
      8, 0.25 -> {
        0.5
      } -. {
        10.0
      }
    }
  }
}
