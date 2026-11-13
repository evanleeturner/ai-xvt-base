/* SPDX-License-Identifier: MIT */
/* z_xvt 004188a0 */
extern unsigned int DAT_007091a4;
extern unsigned short DAT_00703c30;
extern unsigned short DAT_007329b8;
extern unsigned short DAT_00523c60[];
extern unsigned int DAT_006db794;
extern unsigned int DAT_0071b70c;
extern unsigned int DAT_007325d0;

void __stdcall FUN_004188a0(void)
{
    unsigned char bVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int local_4;

    bVar1 = 0;
    uVar3 = DAT_007091a4;
    if ((int)DAT_007091a4 < 0) {
        uVar3 = 0u - DAT_007091a4;
        bVar1 = 0x80;
    }
    uVar3 = (uVar3 >> 0x10) *
            (unsigned int)DAT_00523c60[(unsigned int)(((DAT_00703c30 >> 5) & 0x3fe) >> 1)] +
            (((uVar3 & 0xffff) *
              (unsigned int)DAT_00523c60[(unsigned int)(((DAT_00703c30 >> 5) & 0x3fe) >> 1)] + 0x8000)
             >> 0x10);
    if (((bVar1 ^ (unsigned char)(DAT_00703c30 >> 8)) & 0x80) != 0) {
        uVar3 = 0u - uVar3;
    }

    bVar1 = 0;
    uVar2 = uVar3;
    if ((int)uVar3 < 0) {
        uVar2 = 0u - uVar3;
        bVar1 = 0x80;
    }
    local_4 = (unsigned int)DAT_00523c60[(unsigned int)(((unsigned short)(DAT_007329b8 + 0x4000) >> 5 & 0x3fe) >> 1)];
    DAT_006db794 = (((uVar2 & 0xffff) * local_4 + 0x8000) >> 0x10) + (uVar2 >> 0x10) * local_4;
    if (((bVar1 ^ (unsigned char)((unsigned short)(DAT_007329b8 + 0x4000) >> 8)) & 0x80) != 0) {
        DAT_006db794 = 0u - DAT_006db794;
    }

    bVar1 = 0;
    if ((int)uVar3 < 0) {
        uVar3 = 0u - uVar3;
        bVar1 = 0x80;
    }
    DAT_0071b70c =
        (((uVar3 & 0xffff) *
              (unsigned int)DAT_00523c60[(unsigned int)(((DAT_007329b8 >> 5) & 0x3fe) >> 1)] + 0x8000)
         >> 0x10) +
        (uVar3 >> 0x10) *
            (unsigned int)DAT_00523c60[(unsigned int)(((DAT_007329b8 >> 5) & 0x3fe) >> 1)];
    if (((bVar1 ^ (unsigned char)(DAT_007329b8 >> 8)) & 0x80) != 0) {
        DAT_0071b70c = 0u - DAT_0071b70c;
    }

    bVar1 = 0;
    uVar3 = DAT_007091a4;
    if ((int)DAT_007091a4 < 0) {
        uVar3 = 0u - DAT_007091a4;
        bVar1 = 0x80;
    }
    DAT_007325d0 =
        (((uVar3 & 0xffff) *
              (unsigned int)DAT_00523c60[(unsigned int)(((unsigned short)(DAT_00703c30 + 0x4000) >> 5 & 0x3fe) >> 1)] +
          0x8000) >> 0x10) +
        (uVar3 >> 0x10) *
            (unsigned int)DAT_00523c60[(unsigned int)(((unsigned short)(DAT_00703c30 + 0x4000) >> 5 & 0x3fe) >> 1)];
    if (((bVar1 ^ (unsigned char)((unsigned short)(DAT_00703c30 + 0x4000) >> 8)) & 0x80) != 0) {
        DAT_007325d0 = 0u - DAT_007325d0;
    }
    return;
}
