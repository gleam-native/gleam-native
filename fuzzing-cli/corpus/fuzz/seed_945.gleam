pub const tag_value: Int = 1
pub const euler_value: Bool = True
pub const seed_value: Bool = True

pub type V0 {
  Cv1(value: List(Int))
}

pub type V2 {
  Ok(Float)
}

pub type V3 {
  Cv4
  None
  Some(value: String)
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: Int, v5: Int, x: Bool) -> Int {
{
    {
      let constructor = [5, 4]
      3 |> spin(4)
    }
  } % 2
}

fn f1(m: V3, v6: V0, v7: String) -> List(Int) {
case 1 + 5 {
    _ -> case <<"res":utf8, 10:8>> {
      <<"a":utf8, _:utf8>> -> {
        let v7 = 10.0
        []
      }
      <<constructor:8, "":utf8>> -> fn(v8) { [] }("x")
      _ -> fn(v9) { [7, 100] }(1)
    }
    9 | 1 -> case <<"b":utf8>> {
      <<3:1>> -> [4]
      _ -> [100]
    }
  }
}

pub fn main() {
  let seed_value = case <<7:8, "constructor":utf8, "data":utf8>>, {
      100.0
    } -. {
      100.0
    } {
    <<_:utf8, _:little-signed-16>>, 0.25 as whole -> []
    <<"res":utf8>>, 0.5 -> []
    _, 3.14 -> [5, 2]
    v10, v11 -> [3, 4]
  }
  echo []
  echo case {
      let v = tag_value
      "a"
    }, {
      let v = "constructor"
      let m = []
      seed_value
    } {
    "abc", [3] -> {
      0.5
    } +. {
      fn(v12) { 0.5 }(0)
    }
    _, [constructor, tag_value, ..] -> {
      1.0
    } +. {
      0.0
    }
    v13, v14 -> case #(100.0, 0.25) {
      #(100.0, v15) as whole -> v15
      #(10.0, _) | #(3.14, 2.0) -> fn(v16, v17) { 3.14 }(False, True)
      item -> {
        0.0
      } *. {
        0.0
      }
    }
  }
  echo {
    let item = f1({
      let this_ = False
      let seed_value = 1
      None
    }, {
      let z = "res"
      let seed_value = tag_value
      Cv1([])
    }, fn(v18) { "ab" }(4))
    "data"
  }
  echo 1.0
}
