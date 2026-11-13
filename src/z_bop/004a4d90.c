/* SPDX-License-Identifier: MIT */
/* z_bop 004a4d90 */
extern void *DAT_009a1fe8;

unsigned int FUN_00425b70(unsigned int param_1, unsigned short param_2);
void FUN_004a4d70(int param_1, short param_2);
unsigned int FUN_00425c20(unsigned short param_1, unsigned short param_2);

void FUN_004a4d90(int param_1, unsigned int param_2)
{
    int iVar1;
    int iVar2;
    unsigned short uVar3;
    unsigned short uVar6;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned short uVar7;

    iVar1 = *(int *)((char *)DAT_009a1fe8 + param_1 * 0x23 + 0x1f);
    iVar2 = *(int *)(iVar1 + 0xa9);
    uVar6 = (unsigned short)((unsigned int)iVar1 >> 0x10);
    uVar3 = (unsigned short)((unsigned short)6 - *(unsigned char *)(iVar2 + 0x219) - *(unsigned char *)(iVar2 + 0x23e) - *(unsigned char *)(iVar2 + 0x21c));
    if (uVar3 < 0x8000) {
        uVar7 = *(unsigned short *)(iVar2 + 0x92);
        uVar4 = FUN_00425b70((uVar6 << 16) | (unsigned short)(uVar3 * 0x2000), uVar7);
        uVar7 = (unsigned short)(uVar7 + (short)uVar4);
    }
    else {
        uVar7 = *(unsigned short *)(iVar2 + 0x92);
        uVar4 = FUN_00425b70((uVar6 << 16) | (unsigned short)(uVar3 * 0xe000), uVar7);
        uVar7 = (unsigned short)(uVar7 - (short)uVar4);
    }
    if (uVar7 <= param_2) {
        FUN_004a4d70(param_1, 0xffff);
        return;
    }
    uVar5 = FUN_00425c20((unsigned short)param_2, uVar7);
    FUN_004a4d70(param_1, (short)uVar5);
    return;
}
