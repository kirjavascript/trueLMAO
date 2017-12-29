    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    tst.l	($A10008).l
    nop
    rts

System_Stack:
