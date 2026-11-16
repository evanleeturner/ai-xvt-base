/* SPDX-License-Identifier: MIT */
/* z_bop 004ef600 */
extern int DAT_0052c818;
extern int DAT_0052c81c;
extern unsigned int DAT_0052c808;
extern unsigned int DAT_0052c80c;
extern unsigned int DAT_0052c82c;
extern int DAT_0052c828;
extern unsigned int DAT_0052c830;
extern unsigned char *PTR_DAT_0052c834;
extern unsigned char *PTR_DAT_0052c838;
extern unsigned int DAT_00aa6080;
extern int DAT_00aa6084;
extern int DAT_00b69cb0;
extern int DAT_00b69cbc;

HWND __stdcall GetFocus(void);
void FUN_004eee70(void);
void FUN_004f0a50(int);
void FUN_004d5b70(unsigned char);
void FUN_004ef8e0(int *, unsigned int, int, unsigned int, int *, unsigned int *);
void FUN_004eecb0(int, int, int, int);
int __stdcall _SmackToBuffer_28(int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int);
void __stdcall _SmackDoFrame_4(int);
int __stdcall _SmackToBufferRect_8(int, int);
void __stdcall _SmackNextFrame_4(int);

void __stdcall FUN_004ef600(void)
{
    int iVar4;
    int iVar5;
    int iVar7;
    int *piVar6;
    unsigned char *puVar2;
    HWND pHVar3;
    unsigned int uVar8;
    int uStack_b4;
    unsigned int uStack_b8;
    /* UNRESOLVED: stale register value in the original, no source in this function */
    unsigned int unaff_ESI = 0;
    /* UNRESOLVED: _SmackToBuffer@28 4th arg (stack slot a8); Ghidra shows no value for it */
    unsigned int uStack_a8 = 0;
    unsigned int local_6c[27]; /* 0x6c bytes, local_6c[0] = dwSize */

    pHVar3 = GetFocus();
    if (pHVar3 == *(HWND *)(DAT_0052c818 + 0x20)) {
        if (*(int *)(DAT_0052c81c + 0x68) != 0) {
            FUN_004eee70();
        }
        local_6c[0] = 0x6c;
        while (1) {
            int *piStack_98 = *(int **)(DAT_0052c818 + 0x10);
            iVar4 = ((int (__stdcall *)(int *, int, unsigned int *, int, int))
                     *(void **)(*piStack_98 + 100))(piStack_98, 0, local_6c, 1, 0);
            if (iVar4 != -0x7789fe3e) break;
            piStack_98 = *(int **)(DAT_0052c818 + 0x10);
            iVar4 = ((int (__stdcall *)(int *))*(void **)(*piStack_98 + 0x6c))(piStack_98);
            if (iVar4 != 0) {
                return;
            }
        }
        _SmackToBuffer_28(DAT_0052c81c, DAT_0052c808, DAT_0052c80c,
                          uStack_a8, *(unsigned int *)(DAT_0052c81c + 8),
                          local_6c[4], DAT_0052c82c);
        _SmackDoFrame_4(DAT_0052c81c);
        DAT_0052c828 = 1;
        {
            int *piSurf = *(int **)(DAT_0052c818 + 0x10);
            ((int (__stdcall *)(int *, unsigned int))*(void **)(*piSurf + 0x80))(piSurf, unaff_ESI);
        }
        FUN_004f0a50(*(int *)(DAT_0052c81c + 0x374));
        /* callback convention assumed __cdecl: no cleanup evidence available */
        if ((*(void **)(DAT_0052c818 + 0x2c) != (void *)0) &&
            (iVar4 = ((int (__cdecl *)(int, int, int))*(void **)(DAT_0052c818 + 0x2c))
                         (*(int *)(DAT_0052c81c + 0x374),
                          (int)((unsigned int)*(int *)(DAT_0052c81c + 0xc) - 1u),
                          *(int *)(DAT_0052c818 + 0x30)), iVar4 == 1)) {
            FUN_004d5b70((unsigned char)DAT_00aa6080);
        }
        if ((DAT_00b69cb0 == 0) && (DAT_00b69cbc == 0)) {
            uVar8 = 0;
            iVar4 = _SmackToBufferRect_8(DAT_0052c81c, 0);
            if (iVar4 != 0) {
                iVar4 = 0;
                do {
                    iVar7 = iVar4;
                    if (*(int *)(DAT_0052c81c + 0x388) != 0) {
                        *(int *)(PTR_DAT_0052c838 + iVar4) = *(int *)(DAT_0052c81c + 0x380);
                        iVar7 = (int)((unsigned int)iVar4 + 0x10u);
                        uVar8 = uVar8 + 1;
                        *(int *)(PTR_DAT_0052c838 + iVar4 + 4) = *(int *)(DAT_0052c81c + 900);
                        *(int *)(PTR_DAT_0052c838 + iVar4 + 8) = *(int *)(DAT_0052c81c + 0x388);
                        *(int *)(PTR_DAT_0052c838 + iVar4 + 0xc) = *(int *)(DAT_0052c81c + 0x38c);
                    }
                    iVar5 = _SmackToBufferRect_8(DAT_0052c81c, 0);
                    iVar4 = iVar7;
                } while (iVar5 != 0);
            }
            FUN_004ef8e0((int *)PTR_DAT_0052c838, uVar8, (int)PTR_DAT_0052c834, DAT_0052c830,
                         &uStack_b4, &uStack_b8);
            while (uStack_b8 != 0) {
                uStack_b8 = uStack_b8 - 1;
                piVar6 = (int *)(uStack_b8 * 0x10 + (unsigned int)uStack_b4);
                FUN_004eecb0(piVar6[0], piVar6[1], piVar6[2], piVar6[3]);
            }
            {
                int *piBack = *(int **)(DAT_0052c818 + 0xc);
                ((int (__stdcall *)(int *, int, int))*(void **)(*piBack + 0x2c))(piBack, 0, 1);
            }
            puVar2 = PTR_DAT_0052c834;
            PTR_DAT_0052c834 = PTR_DAT_0052c838;
            PTR_DAT_0052c838 = puVar2;
            DAT_0052c830 = uVar8;
        } else {
            iVar4 = _SmackToBufferRect_8(DAT_0052c81c, 0);
            while (iVar4 != 0) {
                FUN_004eecb0(*(int *)(DAT_0052c81c + 0x380), *(int *)(DAT_0052c81c + 900),
                             *(int *)(DAT_0052c81c + 0x388), *(int *)(DAT_0052c81c + 0x38c));
                iVar4 = _SmackToBufferRect_8(DAT_0052c81c, 0);
            }
        }
        if ((int)((unsigned int)*(int *)(DAT_0052c81c + 0xc) -
                  (unsigned int)*(int *)(DAT_0052c81c + 0x374)) == 1) {
            DAT_00aa6084 = 1;
            return;
        }
        _SmackNextFrame_4(DAT_0052c81c);
    }
    return;
}
