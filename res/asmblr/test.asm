    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    tst.l	4(a0)
    ; tst.l	4(a3,d2.l)
    nop
    rts

System_Stack:
