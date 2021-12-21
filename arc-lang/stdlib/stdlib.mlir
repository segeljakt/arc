
func @add_si32(%a: si32, %b: si32) -> si32 {
  %c = arc.addi %a, %b : si32
  return %c : si32
}

func @sub_si32(%a: si32, %b: si32) -> si32 {
  %c = arc.subi %a, %b : si32
  return %c : si32
}

func @mul_si32(%a: si32, %b: si32) -> si32 {
  %c = arc.muli %a, %b : si32
  return %c : si32
}

func @div_si32(%a: si32, %b: si32) -> si32 {
  %c = arc.divi %a, %b : si32
  return %c : si32
}

