/* SPDX-License-Identifier: MIT */
/* z_bop 00485f00 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);

extern unsigned short DAT_00612298_2;
extern unsigned int   _DAT_00612298;
extern unsigned short _DAT_006122a0;
extern unsigned int   _DAT_00612b28;
extern unsigned int   _DAT_00612b2c;
extern unsigned int   _DAT_00612b10;
extern unsigned int   _DAT_00612b08;
extern unsigned int   _DAT_00612b0c;
extern unsigned int   DAT_00612adc;
extern unsigned char  DAT_00612ae0;
extern unsigned int   DAT_0061228c;

extern unsigned int  *DAT_00999420;
extern unsigned short DAT_00999424;
extern int            DAT_00999426;   /* misaligned */
extern unsigned int  *DAT_0099942a;   /* misaligned */
extern unsigned int  *DAT_0099942e;   /* misaligned */
extern unsigned int   DAT_00999432;   /* misaligned */
extern unsigned short DAT_00999436;
extern unsigned int   DAT_00999438;
extern unsigned int   DAT_0099943c;
extern unsigned int   DAT_00999440;
extern unsigned short DAT_00999444;
extern unsigned int   DAT_00999446;   /* misaligned */
extern unsigned int   DAT_0099944a;   /* misaligned */
extern unsigned short DAT_0099944e;
extern unsigned int   DAT_00999450;
extern unsigned int   DAT_00999454;
extern unsigned int   DAT_0099945c;
extern unsigned short DAT_00999460;
extern unsigned int   DAT_0099946a;   /* misaligned */
extern unsigned short DAT_0099946e;
extern unsigned int   DAT_00999478;
extern unsigned short DAT_0099947c;
extern unsigned int   DAT_00999482;   /* misaligned */
extern unsigned short DAT_00999486;
extern unsigned int   _DAT_0099948c;
extern unsigned short DAT_00999490;
extern unsigned int   DAT_00999492;   /* misaligned */
extern unsigned short DAT_00999496;
extern unsigned int   DAT_00999498;
extern unsigned short DAT_0099949c;
extern unsigned int   DAT_009994a2;   /* misaligned */
extern float          _DAT_0099940c;
extern unsigned int   DAT_00999410;

extern unsigned short DAT_009a1fe4;
extern unsigned int   DAT_009d682c;
extern unsigned short DAT_0052747c;
extern unsigned int   DAT_009a8074;
extern float          _DAT_005181f4;
extern int            DAT_00a081ec;
extern int            DAT_00527e90;

/* one-argument __thiscall == __fastcall (ECX only) */
unsigned int __fastcall FUN_004ac720(unsigned int ecx_arg);
void FUN_0040b180(void);

void __fastcall FUN_00485f00(void *this, int unused_edx, int param_1)
{
    unsigned int  *puVar1;
    unsigned char  bVar2;
    unsigned int   uVar3;
    /* upper halves of ECX left over at each call: unresolved, 0 placeholder */
    unsigned short extraout_var = 0;
    unsigned short extraout_var_00 = 0;
    unsigned short extraout_var_01 = 0;
    unsigned short extraout_var_02 = 0;
    unsigned short extraout_var_03 = 0;
    unsigned short extraout_var_04 = 0;
    unsigned short extraout_var_05 = 0;
    unsigned short extraout_var_06 = 0;
    unsigned short extraout_var_07 = 0;
    unsigned short extraout_var_08 = 0;
    int            iVar4;
    unsigned char *pbVar5;
    unsigned char *pbVar6;
    unsigned char  bVar7;
    unsigned int  *puVar8;
    unsigned int   uVar9;
    unsigned short in_FPUControlWord = 0; /* unresolved FPU control word */
    unsigned int   local_c;
    unsigned int   uStore;
    unsigned int  *pStore;
    unsigned int   uBase;
    int            iCnt;

    _DAT_00612298 = ((unsigned int)DAT_00612298_2 << 16) | in_FPUControlWord;
    _DAT_00612298 = _DAT_00612298 & 0xfffffcffu;
    _DAT_00612b28 = 0x749dc5aeu;
    _DAT_00612b2c = 0x749dc5aeu;
    _DAT_00612b10 = 0x749dc5aeu;
    _DAT_00612b08 = 0;
    _DAT_00612b0c = 0;
    _DAT_006122a0 = in_FPUControlWord;

    DAT_00999420 = (unsigned int *)FUN_004ac720((((unsigned int)this >> 16) << 16) | DAT_00999424);

    uStore = FUN_004ac720(((unsigned int)extraout_var << 16) | DAT_00999436);
    __builtin_memcpy(&DAT_00999432, &uStore, 4);

    uStore = FUN_004ac720(((unsigned int)extraout_var_00 << 16) | DAT_0099944e);
    __builtin_memcpy(&DAT_0099944a, &uStore, 4);

    DAT_0099945c = FUN_004ac720(((unsigned int)extraout_var_01 << 16) | DAT_00999460);

    uStore = FUN_004ac720(((unsigned int)extraout_var_02 << 16) | DAT_0099946e);
    __builtin_memcpy(&DAT_0099946a, &uStore, 4);

    DAT_00999478 = FUN_004ac720(((unsigned int)extraout_var_03 << 16) | DAT_0099947c);

    uStore = FUN_004ac720(((unsigned int)extraout_var_04 << 16) | DAT_00999486);
    __builtin_memcpy(&DAT_00999482, &uStore, 4);

    _DAT_0099948c = FUN_004ac720(((unsigned int)extraout_var_05 << 16) | DAT_00999490);

    uStore = FUN_004ac720(((unsigned int)extraout_var_06 << 16) | DAT_00999496);
    __builtin_memcpy(&DAT_00999492, &uStore, 4);

    DAT_00999440 = FUN_004ac720(((unsigned int)extraout_var_07 << 16) | DAT_00999444);

    DAT_00999498 = FUN_004ac720(((unsigned int)extraout_var_08 << 16) | DAT_0099949c);

    if (param_1 == 0) {
        DAT_00999454 = DAT_00999450;
    } else {
        DAT_00999454 = 0;
        DAT_0099943c = DAT_00999438;
        pStore = DAT_00999420;
        __builtin_memcpy(&DAT_0099942a, &pStore, 4);
        DAT_00999450 = 0;
        uStore = 0;
        __builtin_memcpy(&DAT_00999446, &uStore, 4);
        __builtin_memcpy(&DAT_009994a2, &uStore, 4);
        __builtin_memcpy(&iCnt, &DAT_00999426, 4);
        uStore = ((unsigned int)iCnt * 6u) - 6u;
        pStore = DAT_00999420 + (int)uStore;
        __builtin_memcpy(&DAT_0099942e, &pStore, 4);
        local_c = 0;
        if (DAT_009a1fe4 != 0) {
            iVar4 = 0;
            pbVar6 = (unsigned char *)((unsigned int)DAT_0052747c + DAT_009a8074);
            do {
                uVar9 = 0;
                pbVar5 = pbVar6 + 1;
                __builtin_memcpy(&uBase, &DAT_00999492, 4);
                *(unsigned int *)(uBase + (unsigned int)iVar4) = 0;
                bVar7 = *pbVar6;
                puVar8 = *(unsigned int **)(uBase + (unsigned int)iVar4);
                pbVar6 = pbVar5;
                if (DAT_009d682c != 0) {
                    do {
                        uVar3 = (unsigned int)*pbVar6;
                        pbVar5 = pbVar6 + 1;
                        if (uVar3 == 0) {
                            uVar3 = (unsigned int)*pbVar5;
                            pbVar5 = pbVar6 + 2;
                            if (uVar3 == 0) {
                                bVar2 = *pbVar5;
                                pbVar5 = pbVar6 + 3;
                                uVar3 = bVar2 + 0x100;
                            }
                            uVar3 = uVar3 + 0xff;
                        }
                        if ((char)bVar7 < 0) {
                            __builtin_memcpy(&pStore, &DAT_0099942a, 4);
                            if (puVar8 == (unsigned int *)0) {
                                *(unsigned int **)(uBase + (unsigned int)iVar4) = pStore;
                            } else {
                                *puVar8 = (unsigned int)pStore;
                            }
                            puVar8 = pStore;
                            puVar1 = pStore + 1;
                            pStore = pStore + 6;
                            __builtin_memcpy(&DAT_0099942a, &pStore, 4);
                            *puVar1 = uVar9;
                            puVar8[2] = uVar9 + uVar3;
                            puVar8[5] = (unsigned int)&DAT_00612ae0;
                            *puVar8 = 0;
                        }
                        bVar7 = (unsigned char)(0u - bVar7);
                        uVar9 = uVar9 + uVar3;
                        pbVar6 = pbVar5;
                    } while (uVar9 < DAT_009d682c);
                }
                iVar4 = (int)((unsigned int)iVar4 + 4u);
                local_c = local_c + 1;
                pbVar6 = pbVar5;
            } while (local_c < DAT_009a1fe4);
        }
    }

    if (DAT_0061228c == 0) {
        RaiseException(0xC0000094UL, 0, 0, NULL);
    } else {
        DAT_00999410 = ((DAT_009d682c - 1) + DAT_0061228c) / DAT_0061228c;
    }
    _DAT_0099940c = _DAT_005181f4 / (float)DAT_00a081ec;
    DAT_00612adc = DAT_00612adc + (unsigned int)DAT_009a1fe4;
    if (DAT_00527e90 != 0) {
        FUN_0040b180();
    }
}
