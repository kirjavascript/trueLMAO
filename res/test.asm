    dc.l System_Stack, EntryPoint
    nop
    nop
    nop
    nop

EntryPoint:
    bhi.s   EntryPoint
    bls.s   EntryPoint
    bcc.s   EntryPoint
    bhi.s   EntryPoint
    bcs.s   EntryPoint
    blo.s   EntryPoint
    bvc.w   EntryPoint
    bvs.w   EntryPoint
    bpl.w   EntryPoint
    bmi.w   EntryPoint
    bge.w   EntryPoint
    blt.w   EntryPoint
    bgt.w   EntryPoint
    ble.w   EntryPoint

    and.b	d1,$4(a0)
    and.w	d1,d0	; does nothing now
    and.l	d1,d0	; does nothing now
    and.w	PortA_Ok(pc,d7.w),d5	; only keep X lower bits
    andi.b	#$F,d0  ; interpreted as move (?)
    andi.w	#3,d0
    andi.w	#6,($FFFF8080).w
    andi.l	#$FFFFFF,d0	; 8x8 tile pointer

    tst.l	($A10008).l	; test ports A and B control
    beq.w       PortA_Ok
    bne.w	EntryPoint	; If so, branch.
    tst.w	($A1000C).l	; test ports A and B control
    bne.w	EntryPoint	; If so, branch.
    lea	        PortA_Ok(pc),a5
    movem.w	(a5)+,d5-d7
    movem.l	(a5)+,a0-a4
PortA_Ok:
    nop
    nop
    nop
    nop
skip:
System_Stack:

    bra.s *-$0
    bra.w EntryPoint
    bra.s *-$4
    bra.w EntryPoint
    bra.s *-$A
    bra.s *-$14

asd:
    move.l	-4(a3, a2.l),d0
    move.l	d0,-4(a3, a2.l)
    clr.b       d0
    move.l      -4(a3, a2.l), 4(a3, a2.l)
    move.l      #3, 4(a3, a2.l)
    move.l      4(a3, a2.l), d6
    move.b      ($A000).l, d0
    move.l      #3, d0
    tst.b       ($A10008).l
    move.l	-4(a3, a2.l),d0
    move.l	d0,-4(a3, a2.l)
    clr.b       d0
    move.l      -4(a3, a2.l), 4(a3, a2.l)
    move.l      #3, 4(a3, a2.l)
    move.l      4(a3, a2.l), d6
    move.b      ($A000).l, d0
    move.l      #3, d0
    tst.b       ($A10008).l
    bra.w       PortA_Ok
    tst.l	(a4, a7.w)
    tst.l	4(a3, a2.l)
    tst.l	4(a0)
    move.l	d0,-4(a3, a2.l)
    tst.l	4(a3, a2.l)
    tst.l	-4(a0)
    movem.l	($A000).l,d0-d3/d5-d6
    movem.l	($A000).l,d0
    movem.l	($A000).l,d0-d7
    movem.w	(a5)+,d5-d7
    movem.l	(a5)+,a0-a4
    movem.l	d0-d3,-(sp)
    movem.l	d0-d7,-(sp)
    movem.l	-4(a3, a2.l),d0-a7
    movem.l	d0-a6,-(sp)
    movem.w	(a5)+,a0
    movem.l	d0-a1/a3-a5,-(sp)
    movem.l	(sp)+,d0-a1/a3-a5
    movem.l	d0-d7,($1000).w

    move.w	-4(pc),d0
    move.l	-4(a3),d0

    move.l	-4(a3, a2.l),d0
    rts

    nop
    rts
