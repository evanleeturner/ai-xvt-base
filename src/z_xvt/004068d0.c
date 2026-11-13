/* SPDX-License-Identifier: MIT */
/* z_xvt 004068d0 */
extern int DAT_00525738;
extern unsigned short DAT_00732e42;

void __cdecl FUN_004068d0(unsigned short param_1, int param_2, unsigned short *param_3)
{
    unsigned short *puVar1;
    unsigned char bVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned short uVar5;
    unsigned short uVar6;

    *param_3 = param_1;
    puVar1 = param_3 + 0x204;
    if (DAT_00525738 == 1) {
        *puVar1 = 0x100;
        param_3[0x205] = 0x100;
        DAT_00732e42 = 0x2000;
    }
    else {
        *puVar1 = 0xe9;
        param_3[0x205] = 0x11a;
        DAT_00732e42 = 0x2200;
    }
    uVar3 = (unsigned int)*(unsigned short *)(param_2 + 10) * (unsigned int)param_1;
    uVar4 = uVar3 >> 0x10;
    *(char *)(param_3 + 2) = (char)(uVar3 >> 0x10);
    *(char *)((int)param_3 + 5) = (char)(uVar3 >> 0x18);
    if (*(short *)(param_2 + 0x18) != 0) {
        *(char *)(param_3 + 2) = (char)(*puVar1 * uVar4 >> 8);
        *(char *)((int)param_3 + 5) = (char)(*puVar1 * uVar4 >> 0x10);
    }
    uVar4 = uVar4 + (*(unsigned short *)(param_2 + 0x1930) * uVar4 >> 8);
    if (*(short *)(param_2 + 0x1a) == 0) {
        uVar4 = param_3[0x205] * uVar4 >> 8;
    }
    bVar2 = (unsigned char)param_3[2];
    *(char *)(param_3 + 3) = (char)uVar4;
    *(char *)((int)param_3 + 7) = (char)(uVar4 >> 8);
    if (((unsigned char)param_3[1] != bVar2) || (*(char *)((int)param_3 + 3) != *(char *)((int)param_3 + 5))) {
        *(unsigned char *)(param_3 + 1) = bVar2;
        *(unsigned char *)((int)param_3 + 3) = *(unsigned char *)((int)param_3 + 5);
        uVar4 = (unsigned int)*(unsigned char *)((int)param_3 + 5) << 0x10 | (unsigned int)bVar2 << 8;
        uVar3 = uVar4;
        uVar5 = 0;
        do {
            uVar6 = uVar5 + 1;
            param_3[uVar5 + 4] = (unsigned short)uVar3;
            param_3[uVar5 + 0x104] = (unsigned short)(uVar3 >> 0x10);
            uVar3 = uVar3 + uVar4;
            uVar5 = uVar6;
        } while (uVar6 < 0x100);
    }
    return;
}
