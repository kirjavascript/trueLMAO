    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    move.l      4(a3, a2.l), 4(a3, a2.l)
    move.l      d6, 4(a3, a2.l)
    move.l      4(a3, a2.l), d6
    move.b      ($A000).l, d0
    move.l      #3, d0
    tst.b       ($A10008).l
    tst.l	(a4, a7.w)
    tst.l	4(a3, a2.l)
    tst.l	4(a0)
    nop
    rts

System_Stack:
