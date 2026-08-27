pub const limit_value: Float = 100.0
pub const golden_value: Bool = True
pub const tag_value: Int = 5

pub type V0 {
  Number(value: String, inner: Float)
  Cv1(value: Float)
}

fn f0(v: Int, v2: #(Float, List(Int))) -> List(Int) {
case <<100:8>>, Cv1(100.0) {
    <<"bc":utf8>>, Number("abc", v3) -> case 0.5, {
        let rest = v
        let y = "bc"
        Cv1(0.1)
      } {
      v4, Cv1(3.14) -> fn(v5, v6) { [10] }("bc", 0.0)
      _, pair -> fn(v7, v8) { [] }("b", True)
      v9, v10 -> fn(v11) { [] }(True)
    }
    <<100:1, _:utf8>>, v2 -> case v2 {
      _ -> fn(v12) { [7] }(3)
      Number(_, 10.0) -> []
    }
    _, Number(_, _) -> case Cv1(3.14) {
      Cv1(inner) -> {
        let class = [7]
        let x = "a"
        class
      }
      _ -> [7]
      _ -> []
    }
    _, v13 -> []
  }
}

fn default(x: V0, v14: Int) -> String {
"a"
}

fn f2(constructor: String) -> String {
{
    default(Cv1(100.0), 0) <> {
      constructor <> constructor
    }
  } <> constructor
}

pub fn main() {
  let golden_value = "constructor"
  echo golden_value |> f2()
}
