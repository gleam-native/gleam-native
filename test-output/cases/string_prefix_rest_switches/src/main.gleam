// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Gleam contributors

// A clause mixing a guarded string-prefix pattern with overlapping string
// literals makes the compiled decision tree switch on the prefix's rest.
// Found by the differential fuzzer: native lowering has no materialized
// subject for a rest variable and must fold the accumulated prefix into
// the base subject's checks instead.
pub fn main() -> Nil {
  echo classify(#("rn", 56))
  echo classify(#("rn", 61))
  echo classify(#("rx", 61))
  echo classify(#("rnx", 56))
  echo classify(#("rz", 56))
  echo classify(#("q", 56))
  echo deep("rna")
  echo deep("rnb")
  echo deep("rn")
  echo deep("rq")
  Nil
}

fn classify(pair: #(String, Int)) -> Int {
  case pair {
    #("r" <> rest, 61) if rest != "x" -> 1
    #("rn", 56) -> 2
    #("r" <> _, 56) -> 3
    _ -> 0
  }
}

fn deep(text: String) -> Int {
  case text {
    "r" <> rest if rest == "q" -> 10
    "rna" -> 11
    "rn" <> _ -> 12
    "r" <> _ -> 13
    _ -> 14
  }
}
