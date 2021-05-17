;----------------------------------------------------------------------------
; vdp.inc
;----------------------------------------------------------------------------

vdp_init:
                        ; Clear any pending interrupts
                        in      a, ($BF)

                        ; Wait around for 8 frames
                        ld      bc, $08BF
wait_vbl                in      a, ($BF)
                        bit     7, a
                        jr      z, wait_vbl
                        djnz    wait_vbl

                        ; Load VDP regs
                        ld      hl, vdp_table
                        ld      bc, $00BF|(vdp_table_len<<8)
                        otir

                        ; Load CRAM
                        ld      hl, $C000
                        rst     10h
                        ld      hl, sms_cram_table
                        ld      bc, $00BE|(sms_cram_table_len<<8)
                        otir

                        ; Clear VRAM
                        ld      hl, $4000
                        rst     10h
                        ld      bc, $4000
clear_vram:             xor     a
                        out     ($BE), a
                        dec     bc
                        ld      a, b
                        or      c
                        jr      nz, clear_vram

                        ld      hl, $7F00
                        rst     10h
                        ld      bc, $40BE
                        ld      a, $D0
clear_satb:             out     (c), a
                        djnz    clear_satb

                        ld      hl, $4000
                        rst     10h

                        ret

sms_cram_table:         .db     $15, $3f, $22, $33, $00, $00, $00, $00,
                        .db     $00, $00, $00, $00, $00, $00, $00, $10,
                        .db     $15, $3f, $22, $33, $00, $00, $00, $00,
                        .db     $00, $00, $00, $00, $00, $00, $00, $10,
sms_cram_table_len      =       $-sms_cram_table
                              
vdp_table:              .dw     $8026        
                        .dw     $81A2   
                        .dw     $82FF
                        .dw     $83FF
                        .dw     $84FF   
                        .dw     $85FF
                        .dw     $86FB
                        .dw     $870F
                        .dw     $8800
                        .dw     $8900
                        .dw     $8AFF
vdp_table_len           =       $-vdp_table
