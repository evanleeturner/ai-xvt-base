/* SPDX-License-Identifier: MIT */
/* z_bop 004d49a0 */
/* FUN_004d6ca0 receives extraout_ECX, an ECX value left over from earlier code
 * in the original binary. Ghidra gives no defined source for it. It is
 * initialised to 0 only to satisfy the build gate (-Werror,-Wuninitialized);
 * the true value is unknown. The callee was not decompiled, so its __fastcall
 * convention is inferred from the decompile and not verified.
 *
 * Ghidra's puStack_48 is local_80[14]. It is the lpSurface slot of the 0x6c-byte
 * surface descriptor that begins at local_80[5] and spans 27 dwords, so
 * local_80 needs 32 entries and puStack_48 is an lvalue alias of local_80[14].
 *
 * Globals at addresses that are not a multiple of their width are kept as byte
 * arrays and accessed only through __builtin_memcpy.
 */

void __stdcall FUN_004d4970(void);
void __fastcall FUN_004d6ca0(int extraout_ECX);
int  __stdcall FUN_004d37e0(void);
void __stdcall FUN_004dca20(void);
void __stdcall FUN_004d4c10(void);

extern char DAT_00aa7b0d;
extern char DAT_00aa7b0e;
extern char DAT_00aa7b24;
extern char DAT_00aa7f9e;
extern int  DAT_00b69cb0;
extern int  DAT_00b69cbc;

extern unsigned char DAT_00aa7b1b[4];
extern unsigned char DAT_00aa7b1f[4];
extern unsigned char DAT_00aa7b2e[4];
extern unsigned char DAT_00aa7b32[4];
extern unsigned char DAT_00aa7b36[4];
extern unsigned char DAT_00aa7b3a[4];
extern unsigned char DAT_00aa7b92[4];
extern unsigned char DAT_00aa7b96[4];

#define VT_SLOT(obj, off) (*(unsigned int *)(*(unsigned int *)(obj) + (off)))

void __stdcall FUN_004d49a0(void)
{
    int iVar1;
    int extraout_ECX = 0;
    unsigned int *puVar2;
    int *piVar3;
    int local_80[32];
    int *tmpA;
    int *tmpB;
#define puStack_48 (*(unsigned int **)&local_80[14])

    __builtin_memcpy(&tmpA, DAT_00aa7b2e, 4);
    if (tmpA != (int *)0) {
        if (DAT_00aa7b0d != '\0') {
            FUN_004d4970();
        }
        local_80[3] = 0x280;
        local_80[4] = 0x1e0;
        local_80[1] = 0;
        local_80[2] = 0;
        if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
            do {
                __builtin_memcpy(&tmpA, DAT_00aa7b2e, 4);
                iVar1 = ((int (__stdcall *)(int *, int *))VT_SLOT(tmpA, 0x44))(tmpA, local_80);
                if (iVar1 != 0) break;
            } while (local_80[0] == 0);
            __builtin_memcpy(&iVar1, DAT_00aa7b1f, 4);
            if ((DAT_00aa7f9e == '\x01') && (iVar1 == 8)) {
                FUN_004d6ca0(extraout_ECX);
                DAT_00aa7f9e = '\0';
            }
            for (;;) {
                __builtin_memcpy(&tmpA, DAT_00aa7b36, 4);
                __builtin_memcpy(&tmpB, DAT_00aa7b3a, 4);
                iVar1 = ((int (__stdcall *)(int *, int *, int))VT_SLOT(tmpA, 0x2c))(tmpA, tmpB, 1);
                if (iVar1 == 0) break;
                if (iVar1 == -0x7789fe3e) {
                    iVar1 = FUN_004d37e0();
                    if (iVar1 != 0) break;
                    continue;
                }
                if (iVar1 == -0x7789fde4) continue;
                break;
            }
        }
        else {
            for (;;) {
                __builtin_memcpy(&tmpA, DAT_00aa7b36, 4);
                __builtin_memcpy(&tmpB, DAT_00aa7b3a, 4);
                iVar1 = ((int (__stdcall *)(int *, int, int, int *, int *, int))VT_SLOT(tmpA, 0x1c))
                            (tmpA, 0, 0, tmpB, local_80 + 1, 0);
                if (iVar1 == 0) break;
                if (iVar1 == -0x7789fe3e) {
                    iVar1 = FUN_004d37e0();
                    if (iVar1 != 0) {
                        return;
                    }
                }
                else if (iVar1 != -0x7789fde4) {
                    return;
                }
            }
        }
        if (DAT_00aa7b24 != '\0') {
            __builtin_memcpy(&iVar1, DAT_00aa7b96, 4);
            if (iVar1 != 0) {
                iVar1 = 0;
                __builtin_memcpy(DAT_00aa7b96, &iVar1, 4);
                __builtin_memcpy(&puVar2, DAT_00aa7b92, 4);
                if (puVar2 != (unsigned int *)0) {
                    __builtin_memcpy(&tmpA, DAT_00aa7b32, 4);
                    if (tmpA != (int *)0) {
                        piVar3 = local_80 + 5;
                        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
                            *piVar3 = 0;
                            piVar3 = piVar3 + 1;
                        }
                        local_80[5] = 0x6c;
                        for (;;) {
                            __builtin_memcpy(&tmpA, DAT_00aa7b32, 4);
                            iVar1 = ((int (__stdcall *)(int *, int, int *, int, int))VT_SLOT(tmpA, 100))
                                        (tmpA, 0, local_80 + 5, 0, 0);
                            if (iVar1 == 0) break;
                            if (iVar1 == -0x7789fe3e) {
                                __builtin_memcpy(&tmpA, DAT_00aa7b32, 4);
                                ((int (__stdcall *)(int *))VT_SLOT(tmpA, 0x6c))(tmpA);
                                continue;
                            }
                            if (iVar1 == -0x7789fde4) continue;
                            break;
                        }
                        if (puStack_48 != (unsigned int *)0) {
                            __builtin_memcpy(&puVar2, DAT_00aa7b92, 4);
                            __builtin_memcpy(&iVar1, DAT_00aa7b1b, 4);
                            for (iVar1 = (int)((((unsigned int)iVar1 * 0xfU) & 0x7ffffffU) << 3);
                                 iVar1 != 0; iVar1 = iVar1 + -1) {
                                *puStack_48 = *puVar2;
                                puVar2 = puVar2 + 1;
                                puStack_48 = puStack_48 + 1;
                            }
                            __builtin_memcpy(&tmpA, DAT_00aa7b32, 4);
                            ((int (__stdcall *)(int *, int))VT_SLOT(tmpA, 0x80))(tmpA, 0);
                        }
                    }
                }
            }
            if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
                for (;;) {
                    __builtin_memcpy(&tmpA, DAT_00aa7b3a, 4);
                    __builtin_memcpy(&tmpB, DAT_00aa7b32, 4);
                    iVar1 = ((int (__stdcall *)(int *, int, int, int *, int *, int))VT_SLOT(tmpA, 0x1c))
                                (tmpA, 0, 0, tmpB, local_80 + 1, 0);
                    if (iVar1 == 0) break;
                    if (iVar1 == -0x7789fe3e) {
                        iVar1 = FUN_004d37e0();
                        if (iVar1 != 0) {
                            return;
                        }
                    }
                    else if (iVar1 != -0x7789fde4) {
                        return;
                    }
                }
            }
            else {
                FUN_004dca20();
            }
        }
        if (DAT_00aa7b0e != '\0') {
            FUN_004d4c10();
        }
    }
    return;
#undef puStack_48
}
