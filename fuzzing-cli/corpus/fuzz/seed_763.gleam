pub type V0 {
  Record(value: String, inner: String)
  Cv1(Int, List(Int))
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(class: String, rest: String, v2: Int) -> Float {
{
    let class = {
      10.0
    } >. {
      {
        let arguments = "ab"
        let l = 1
        1.0
      }
    }
    let class = fn(v3, v4) { rest }(10, True)
    fn(v5) { {
      1.5
    } +. {
      3.14
    } }("b")
  }
}

fn constructor(v6: Int, v7: Int, arguments: Int) -> String {
"bc"
}

fn f2(l: String, arguments: String, v8: Bool) -> Bool {
True
}

pub fn main() {
  let this_ = 0.1
  echo {
    fn(v9, v10) { "data" }(False, 10.0)
  } |> f2("bc", f2("bc", "x", True))
  echo False
  echo 1
  echo [100, 4]
}
