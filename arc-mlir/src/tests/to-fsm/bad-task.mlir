// RUN: arc-mlir %s -split-input-file -verify-diagnostics

module @toplevel {
  func @my_handler(%this : !arc.struct<x : si32>,
                   %in   : !arc.enum<A : si32, B : si32>,
                   %out  : !arc.stream<!arc.enum<C : si32, D : si32>>)
                -> ()
                attributes { "arc.mod_name" = "my_task",
                             "arc.task_name" = "MyTask"}
  {
    // expected-error@+2 {{'arc.receive' op can only be used inside an event handler}}
    // expected-note@+1 {{see current operation:}}
    %event = arc.receive %in : !arc.enum<A : si32, B : si32>
    %isA = arc.enum_check (%event : !arc.enum<A : si32, B : si32>) is "A" : i1
    "arc.if"(%isA) ( {
      %a = arc.enum_access "A" in (%event : !arc.enum<A : si32, B : si32>) : si32
      %e = arc.make_enum (%a : si32) as "C" : !arc.enum<C : si32, D : si32>
      "arc.emit" (%e, %out) : (!arc.enum<C : si32, D : si32>, !arc.stream<!arc.enum<C : si32, D : si32>>) -> ()
      "arc.block.result"(%isA) : (i1) -> ()
    },  {
      %b = arc.enum_access "B" in (%event : !arc.enum<A : si32, B : si32>) : si32
      %e = arc.make_enum (%b : si32) as "D" : !arc.enum<C : si32, D : si32>
      "arc.emit" (%e, %out) : (!arc.enum<C : si32, D : si32>, !arc.stream<!arc.enum<C : si32, D : si32>>) -> ()
      "arc.block.result"(%isA) : (i1) -> ()
    }) : (i1) -> (i1)
    return
  }

  func @init(%this : !arc.struct<x : si32>) -> ()
                attributes { "arc.mod_name" = "my_task",
                             "arc.task_name" = "MyTask",
			     "arc.is_init"}
  {
    return
  }
}

// -----
module @toplevel {
  func @my_handler(%this : !arc.struct<x : si32>,
                   %in   : !arc.enum<A : si32, B : si32>,
                   %out  : !arc.stream<!arc.enum<C : si32, D : si32>>)
                -> ()
                attributes { "arc.mod_name" = "my_task",
                             "arc.task_name" = "MyTask",
			     "arc.is_event_handler"}
  {
    %dummy = arc.constant 4711 : si32
    %bad = arc.make_enum (%dummy : si32) as "A" : !arc.enum<A : si32, B : si32>
    // expected-error@+2 {{'arc.receive' op event argument is not the handler event}}
    // expected-note@+1 {{see current operation:}}
    %event = arc.receive %bad : !arc.enum<A : si32, B : si32>
    %isA = arc.enum_check (%event : !arc.enum<A : si32, B : si32>) is "A" : i1
    "arc.if"(%isA) ( {
      %a = arc.enum_access "A" in (%event : !arc.enum<A : si32, B : si32>) : si32
      %e = arc.make_enum (%a : si32) as "C" : !arc.enum<C : si32, D : si32>
      "arc.emit" (%e, %out) : (!arc.enum<C : si32, D : si32>, !arc.stream<!arc.enum<C : si32, D : si32>>) -> ()
      "arc.block.result"(%isA) : (i1) -> ()
    },  {
      %b = arc.enum_access "B" in (%event : !arc.enum<A : si32, B : si32>) : si32
      %e = arc.make_enum (%b : si32) as "D" : !arc.enum<C : si32, D : si32>
      "arc.emit" (%e, %out) : (!arc.enum<C : si32, D : si32>, !arc.stream<!arc.enum<C : si32, D : si32>>) -> ()
      "arc.block.result"(%isA) : (i1) -> ()
    }) : (i1) -> (i1)
    return
  }

  func @init(%this : !arc.struct<x : si32>) -> ()
                attributes { "arc.mod_name" = "my_task",
                             "arc.task_name" = "MyTask",
			     "arc.is_init"}
  {
    return
  }
}
