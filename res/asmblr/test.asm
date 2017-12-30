    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    ; tst.b       ($A10008).l
    tst.l	$7F(a3,d3.l)
    ; tst.l	4(a3,a2.l)
    ; tst.l	4(a0)
    nop
    rts

System_Stack:
