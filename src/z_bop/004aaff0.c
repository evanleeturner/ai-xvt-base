/* SPDX-License-Identifier: MIT */
/* z_bop 004aaff0 */
typedef unsigned int undefined4;

extern int FUN_004d56a0(void);
extern undefined4 FUN_004ab970(int param_1);
extern void FUN_004ac400(int *param_1);
extern int FUN_004aafe0(void);
extern void FUN_00408170(void);

extern int *DAT_0066ddd4;
extern undefined4 DAT_0066ddd0;
extern int DAT_00527ea8;
extern int DAT_00527e90;
extern int DAT_005233d8;
extern int *DAT_0066ddc4;
extern int DAT_0066dde0;
extern int _DAT_00803b74;
extern undefined4 DAT_00803b70;
extern int DAT_00527eac;
extern int DAT_00527eb8;
extern int *DAT_0066e700;
extern undefined4 DAT_009d8c08;
extern int *DAT_00803f80;
extern int *_DAT_0066ddec;
extern int *DAT_0066ddcc;
extern undefined4 *DAT_005233e0;
extern undefined4 *DAT_005233e8;
extern int DAT_0066ddd8;

undefined4 __stdcall FUN_004aaff0(void)
{
    int iVar1;
    undefined4 uVar2;
    int iVar3;
    char *pcVar4;
    char *pcVar5;
    char cVar6;
    unsigned int *puVar7;
    int **ppiVar8;
    undefined4 *puVar9;

    /*
     * One contiguous stack frame, base = Ghidra stack offset -0x614 (&stack0xfffff9ec).
     * Dword index = (offset + 0x614) / 4:
     *   frame[0]   = &stack0xfffff9ec (second CreateSurface descriptor)
     *   frame[2]   = puStack_60c
     *   frame[3]   = puStack_608
     *   frame[5]   = apiStack_600[0] (GetSurfaceDesc descriptor base)
     *   frame[9]   = auStack_5f0[0]  (first CreateSurface descriptor base)
     *   frame[22]  = uStack_5bc
     *   frame[26]  = uStack_5ac
     *   frame[37]  = uStack_580
     *   frame[39]  = auStack_578[0]  (caps block)
     *   frame[121] = acStack_430
     */
    unsigned int frame[392];
    int **apiStack_600 = (int **)&frame[5];
    undefined4 *auStack_5f0 = &frame[9];
    unsigned int *auStack_578 = &frame[39];
    char *acStack_430 = (char *)&frame[121];

    DAT_0066ddd4 = (int *)FUN_004d56a0();
    if (DAT_00527ea8 == 0) {
        iVar1 = ((int (__stdcall *)(int *, undefined4, int))(*(int **)DAT_0066ddd4)[0x50 / 4])(DAT_0066ddd4, DAT_0066ddd0, 8);
        if (iVar1 != 0) {
            uVar2 = FUN_004ab970(2);
            return uVar2;
        }
    }
    else {
        iVar1 = ((int (__stdcall *)(int *, undefined4, int))(*(int **)DAT_0066ddd4)[0x50 / 4])(DAT_0066ddd4, DAT_0066ddd0, 0x51);
        if (iVar1 != 0) {
            uVar2 = FUN_004ab970(2);
            return uVar2;
        }
    }
    if (DAT_00527e90 != 0) {
        DAT_005233d8 = 2;
    }
    if (DAT_00527ea8 == 0) goto LAB_004ab425;
    iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, DAT_0066ddc4, DAT_0066dde0);
    if (iVar1 == 0) goto LAB_004ab425;
    if (DAT_0066ddc4 == (int *)0x140) {
        DAT_0066ddc4 = (int *)0x200;
        DAT_0066dde0 = 0x180;
        iVar3 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x200, 0x180);
        iVar1 = 0;
        if (iVar3 != 0) {
            DAT_0066ddc4 = (int *)0x280;
            DAT_0066dde0 = 0x1e0;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
            if (iVar1 != 0) {
                DAT_0066dde0 = 0xf0;
                DAT_0066ddc4 = (int *)0x140;
                goto LAB_004ab1a3;
            }
        }
LAB_004ab2d8:
        if (iVar1 == 0) goto LAB_004ab425;
        if (DAT_005233d8 == 1) {
            DAT_005233d8 = 2;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, DAT_0066ddc4, DAT_0066dde0);
            if (iVar1 == 0) goto LAB_004ab425;
            if (DAT_0066ddc4 == (int *)0x140) {
                DAT_0066ddc4 = (int *)0x200;
                DAT_0066dde0 = 0x180;
                iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x200, 0x180);
                if (iVar1 == 0) goto LAB_004ab425;
                DAT_0066ddc4 = (int *)0x280;
                DAT_0066dde0 = 0x1e0;
                iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
                if (iVar1 == 0) goto LAB_004ab425;
                DAT_0066dde0 = 0xf0;
                DAT_0066ddc4 = (int *)0x140;
            }
            else if (DAT_0066ddc4 == (int *)0x200) {
                DAT_0066ddc4 = (int *)0x280;
                DAT_0066dde0 = 0x1e0;
                iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
joined_r0x004ab2cd:
                if (iVar1 == 0) goto LAB_004ab425;
                DAT_0066dde0 = 0x180;
                DAT_0066ddc4 = (int *)0x200;
            }
        }
    }
    else {
        if (DAT_0066ddc4 == (int *)0x200) {
            DAT_0066ddc4 = (int *)0x280;
            DAT_0066dde0 = 0x1e0;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
            if (iVar1 == 0) goto LAB_004ab2d8;
            DAT_0066dde0 = 0x180;
            DAT_0066ddc4 = (int *)0x200;
        }
LAB_004ab1a3:
        if (iVar1 == 0) goto LAB_004ab40d;
        if (DAT_005233d8 != 2) goto LAB_004ab2d8;
        DAT_005233d8 = 1;
        iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, DAT_0066ddc4, DAT_0066dde0);
        if (iVar1 == 0) goto LAB_004ab425;
        if (DAT_0066ddc4 == (int *)0x140) {
            DAT_0066ddc4 = (int *)0x200;
            DAT_0066dde0 = 0x180;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x200, 0x180);
            if (iVar1 == 0) goto LAB_004ab425;
            DAT_0066ddc4 = (int *)0x280;
            DAT_0066dde0 = 0x1e0;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
            if (iVar1 == 0) goto LAB_004ab425;
            DAT_0066dde0 = 0xf0;
            DAT_0066ddc4 = (int *)0x140;
        }
        else if (DAT_0066ddc4 == (int *)0x200) {
            DAT_0066ddc4 = (int *)0x280;
            DAT_0066dde0 = 0x1e0;
            iVar1 = ((int (__stdcall *)(int *, int *, int))(*(int **)DAT_0066ddd4)[0x54 / 4])(DAT_0066ddd4, (int *)0x280, 0x1e0);
            goto joined_r0x004ab2cd;
        }
    }
LAB_004ab40d:
    if (iVar1 != 0) {
        uVar2 = FUN_004ab970(3);
        return uVar2;
    }
LAB_004ab425:
    if (DAT_005233d8 != 2) {
        DAT_00527e90 = 0;
    }
    uVar2 = 0;
    _DAT_00803b74 = 1;
    puVar7 = auStack_578;
    for (iVar1 = 0x5b; iVar1 != 0; iVar1 = iVar1 + -1) {
        *puVar7 = 0;
        puVar7 = puVar7 + 1;
    }
    auStack_578[0] = 0x16c;
    iVar1 = ((int (__stdcall *)(int *, unsigned int *))(*(int **)DAT_0066ddd4)[0x2c / 4])(DAT_0066ddd4, auStack_578);
    if ((((iVar1 == 0) && ((frame[37] & 0x400000) != 0)) && ((auStack_578[0] & 1) != 0)) &&
        ((auStack_578[0] & 0x200) == 0)) {
        _DAT_00803b74 = 2;
    }
    if (DAT_00527ea8 == 0) {
        puVar9 = auStack_5f0;
        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
            *puVar9 = 0;
            puVar9 = puVar9 + 1;
        }
        auStack_5f0[0] = 0x6c;
        auStack_5f0[1] = 1;
        iVar1 = ((int (__stdcall *)(int *, undefined4 *, int **, int))(*(int **)DAT_0066ddd4)[0x18 / 4])(DAT_0066ddd4, auStack_5f0, &DAT_0066e700, 0);
        if (iVar1 != 0) {
            uVar2 = FUN_004ab970(4);
            return uVar2;
        }
        iVar1 = FUN_004aafe0();
        if (iVar1 == 0) {
            uVar2 = FUN_004ab970(0xc);
            return uVar2;
        }
        ppiVar8 = apiStack_600;
        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
            *ppiVar8 = (int *)0x0;
            ppiVar8 = ppiVar8 + 1;
        }
        apiStack_600[0] = (int *)0x6c;
        ((int (__stdcall *)(int *, int **))(*(int **)DAT_0066e700)[0x58 / 4])(DAT_0066e700, apiStack_600);
        DAT_00803b70 = uVar2;
        if (((frame[22] & 0x20) == 0) && ((frame[22] & 0x40) != 0)) {
            DAT_005233d8 = 2;
            if ((frame[26] & 0x400) == 0) {
                DAT_00527eb8 = 0x22b;
            }
            else {
                DAT_00527eb8 = 0x235;
            }
        }
        else {
            DAT_005233d8 = 1;
            DAT_00527eb8 = 8;
        }
        if (DAT_0066e700 != (int *)0x0) {
            ((int (__stdcall *)(int *))(*(int **)DAT_0066e700)[8 / 4])(DAT_0066e700);
            DAT_0066e700 = (int *)0x0;
        }
    }
    else {
        puVar9 = auStack_5f0;
        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
            *puVar9 = 0;
            puVar9 = puVar9 + 1;
        }
        auStack_5f0[0] = 0x6c;
        if (DAT_00527eac == 0) {
            auStack_5f0[1] = 1;
        }
        else {
            auStack_5f0[1] = 0x21;
        }
        iVar1 = ((int (__stdcall *)(int *, undefined4 *, int **, int))(*(int **)DAT_0066ddd4)[0x18 / 4])(DAT_0066ddd4, auStack_5f0, &DAT_0066e700, 0);
        if (iVar1 != 0) {
            uVar2 = FUN_004ab970(4);
            return uVar2;
        }
        DAT_00527eb8 = 0x235;
        if (DAT_005233d8 != 2) {
            DAT_00527eb8 = 8;
        }
        ppiVar8 = apiStack_600;
        for (iVar1 = 0x1b; iVar1 != 0; iVar1 = iVar1 + -1) {
            *ppiVar8 = (int *)0x0;
            ppiVar8 = ppiVar8 + 1;
        }
        apiStack_600[0] = (int *)0x6c;
        ((int (__stdcall *)(int *, int **))(*(int **)DAT_0066e700)[0x58 / 4])(DAT_0066e700, apiStack_600);
        DAT_00803b70 = uVar2;
        if ((frame[22] & 0x20) == 0) {
            if ((frame[22] & 0x40) != 0) {
                DAT_005233d8 = 2;
                DAT_00527eb8 = 0x235;
                if ((frame[26] & 0x400) == 0) {
                    DAT_00527eb8 = 0x22b;
                }
            }
        }
        else {
            DAT_005233d8 = 1;
            DAT_00527eb8 = 8;
        }
        DAT_009d8c08 = DAT_00803b70;
        FUN_004ac400(DAT_0066e700);
        if (DAT_00527eac == 0) {
            _DAT_0066ddec = DAT_0066e700;
            DAT_00803f80 = DAT_0066e700;
        }
        else {
            frame[2] = 4;
            iVar1 = ((int (__stdcall *)(int *, unsigned int *, int **))(*(int **)DAT_0066e700)[0x30 / 4])(DAT_0066e700, &frame[2], &DAT_00803f80);
            if (iVar1 != 0) {
                uVar2 = FUN_004ab970(5);
                return uVar2;
            }
            frame[3] = (unsigned int)DAT_005233e0;
            frame[2] = (unsigned int)DAT_005233e8;
            iVar1 = ((int (__stdcall *)(int *, unsigned int *, int **, int))(*(int **)DAT_0066ddd4)[0x18 / 4])(DAT_0066ddd4, &frame[0], &DAT_0066ddcc, 0);
            if (iVar1 != 0) {
                uVar2 = FUN_004ab970(6);
                return uVar2;
            }
            FUN_004ac400(DAT_00803f80);
            _DAT_0066ddec = DAT_00803f80;
            FUN_004ac400(DAT_0066ddcc);
        }
        iVar1 = FUN_004aafe0();
        if (iVar1 == 0) {
            uVar2 = FUN_004ab970(0xc);
            return uVar2;
        }
    }
    if (DAT_00527ea8 != 0) {
        if (DAT_005233d8 == 1) {
            cVar6 = '\0';
            pcVar4 = acStack_430;
            do {
                *pcVar4 = cVar6;
                pcVar5 = pcVar4 + 4;
                pcVar4[1] = cVar6;
                pcVar4[2] = cVar6;
                cVar6 = (char)(cVar6 + 1);
                pcVar4 = pcVar5;
            } while (pcVar5 < acStack_430 + 0x400);
            ((int (__stdcall *)(int *, int, char *, int *, int))(*(int **)DAT_0066ddd4)[0x14 / 4])(DAT_0066ddd4, 0x4c, acStack_430, &DAT_0066ddd8, 0);
            if (DAT_0066ddd8 != 0) {
                ((int (__stdcall *)(int *, int))(*(int **)DAT_0066e700)[0x7c / 4])(DAT_0066e700, DAT_0066ddd8);
            }
        }
        else {
            DAT_0066ddd8 = 0;
        }
    }
    if (DAT_00527e90 != 0) {
        FUN_00408170();
    }
    return 1;
}
