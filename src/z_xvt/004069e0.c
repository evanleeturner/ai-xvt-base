/* SPDX-License-Identifier: MIT */
/* z_xvt 004069e0 */
extern unsigned short DAT_006db6ca;
extern unsigned short DAT_006db6ce;
extern short DAT_006da4e8;
extern short DAT_006da4e0;

void __cdecl FUN_004069e0(int param_1, unsigned short *param_2)
{
    unsigned short uVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;

    uVar2 = DAT_006db6ce;
    uVar1 = DAT_006db6ca;
    if ((short)DAT_006db6ca < 0) {
        DAT_006db6ca = (unsigned short)(0u - (unsigned int)DAT_006db6ca);
    }
    if ((short)DAT_006db6ce < 0) {
        DAT_006db6ce = (unsigned short)(0u - (unsigned int)DAT_006db6ce);
    }
    uVar6 = (unsigned int)(((int)((unsigned int)*param_2 * (unsigned int)(int)(short)DAT_006db6ca + 0x80u)) >> 8) & 0xffffu;
    iVar5 = (int)((unsigned int)*(unsigned short *)(param_1 + 6) * uVar6);
    if (((*(unsigned short *)(param_1 + 8) ^ uVar1) & 0x8000) != 0) {
        iVar5 = (int)(0u - (unsigned int)iVar5);
    }
    uVar3 = (unsigned int)(((int)((unsigned int)(((int)((unsigned int)param_2[0x205] * (unsigned int)(int)(short)DAT_006db6ce + 0x80u)) >> 8) * (unsigned int)*param_2 + 0x80u)) >> 8) & 0xffffu;
    iVar4 = (int)((unsigned int)*(unsigned short *)(param_1 + 2) * uVar3);
    if (((*(unsigned short *)(param_1 + 4) ^ uVar2) & 0x8000) != 0) {
        iVar4 = (int)(0u - (unsigned int)iVar4);
    }
    DAT_006da4e8 = (short)(((unsigned int)iVar5 + (unsigned int)iVar4 + 0x8000u) >> 0x10);
    iVar5 = (int)((unsigned int)*(unsigned short *)(param_1 + 2) * uVar6);
    if (((*(unsigned short *)(param_1 + 4) ^ uVar1) & 0x8000) != 0) {
        iVar5 = (int)(0u - (unsigned int)iVar5);
    }
    iVar4 = (int)((unsigned int)*(unsigned short *)(param_1 + 6) * uVar3);
    if (((*(unsigned short *)(param_1 + 8) ^ uVar2) & 0x8000) == 0) {
        iVar4 = (int)(0u - (unsigned int)iVar4);
    }
    iVar4 = (int)((unsigned int)iVar5 + 0x8000u + (unsigned int)iVar4);
    DAT_006da4e0 = (short)((unsigned int)iVar4 >> 0x10);
    if (iVar4 < 0) {
        DAT_006da4e0 = (short)(0u - (unsigned int)(int)DAT_006da4e0);
    }
    DAT_006da4e0 = (short)(((unsigned int)param_2[0x204] * (unsigned int)(int)DAT_006da4e0 + 0x80u) >> 8);
    if (iVar4 < 0) {
        DAT_006da4e0 = (short)(0u - (unsigned int)(int)DAT_006da4e0);
    }
    return;
}
