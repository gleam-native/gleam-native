pub const limit_value: String = "data"
pub const euler_value: String = "abc"
pub const tag_value: Int = 100

fn f0(m: String) -> List(Int) {
[]
}

fn f1(m: Bool, v0: Float, v1: Int) -> List(Int) {
[]
}

pub fn main() {
  let euler_value = {
    let default = "x"
    let item = 5
    [7]
  }
  echo f1(case {
      let arguments = limit_value
      #(42, False)
    }, "x" {
    #(1, _) as whole, "data" -> True
    #(0, acc), "ab" <> rest if !acc -> True
    #(7, v2) as whole, "bc" <> _ -> True || v2
    _, _ -> True
  }, {
    {
      0.0
    } +. {
      0.5
    }
  } /. {
    3.14
  }, case {
      let v = True
      #("res", True)
    }, {
      let limit_value = 0.5
      tag_value
    } {
    #(_, False), tag_value -> tag_value
    #("constructor" <> rest as whole, _), _ -> tag_value
    _, _ -> tag_value + tag_value
  })
  echo fn(v3) { {
    let limit_value = 1
    v3 *. {
      0.0
    }
  } }(0.0)
}
