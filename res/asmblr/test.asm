    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    tst.l	#$FFFFFFFF
    nop
    rts

System_Stack:
