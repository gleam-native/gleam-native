pub type Record {
  Cv0(value: String, inner: Int)
  Cv1(Bool)
  Cv2
}

pub type Promise {
  Cv3(Bool, value: List(Int))
  Cv4(value: List(Int))
  Cv5
}

fn f0(v6: Float, v7: Int, v8: Int) -> Float {
v6
}

fn f1(default: Int) -> Float {
case [42] {
    [a] if a > 1 && a > 3 -> 2.0
    [4] -> {
      let item = {
        let default = [2]
        "b"
      }
      let l = 7
      1.5
    }
    [] -> {
      3.14
    } -. {
      0.1
    }
    _ -> {
      10.0
    } |> f0(0 - default, fn(v9, v10) { 1 }(1, 4))
  }
}

fn f2(default: Bool, item: #(Float, Float), v11: Promise) -> List(Int) {
case [7], <<"b":utf8>> {
    [3, b, ..] as whole, <<_:utf8, _:8, _:utf8>> as it if b % 2 == 0 -> [2]
    [b, 2, ..], <<7:8, _:8>> as whole -> {
      let whole = fn(v12, v13) { default }(10, "res")
      []
    }
    [], _ -> [4]
    _, _ -> []
  }
}

pub fn main() {
  let class = 5
  let delete = f2(True || False, #(0.25, 10.0), Cv3(True, [42, 5]))
  echo {
    let self_ = {
      1 + 4
    } == 5
    let y = {
      {
        let y = 5
        0.0
      }
    } >=. {
      {
        0.1
      } -. {
        1.5
      }
    }
    delete
  }
  echo case fn(v14) { Cv5 }(42) {
    Cv3(v15, [4, _, ..] as whole) if v15 && v15 -> delete
    a -> [0]
  }
  echo False
}
