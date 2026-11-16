/* SPDX-License-Identifier: MIT */
/* z_xvt 00429310 */
extern int DAT_00732e54;
extern char *DAT_00732a40[];
extern unsigned short DAT_006d9d50;
extern unsigned int DAT_006d4988;
extern unsigned char DAT_006d9d56[4];
extern unsigned char DAT_0070b1f2[];
extern unsigned char DAT_0070b1f4[];
extern unsigned char DAT_0070b1da[];
extern unsigned int DAT_00732a2c;
extern unsigned int DAT_00732a28;
extern unsigned int DAT_00732a24;

int __cdecl FUN_0042b150(int param_1);
void __cdecl FUN_00421c70(unsigned int param_1, unsigned int param_2);
void __cdecl FUN_00491ad0(void);

void __cdecl FUN_00429310(unsigned int param_1)
{
    char cVar1;
    char *pcVar2;
    unsigned char *iVar3;
    unsigned char *pTmp;
    unsigned int uTmp;
    int iTmp;

    iVar3 = (unsigned char *)(unsigned int)FUN_0042b150(DAT_00732e54);
    pcVar2 = DAT_00732a40[*(iVar3 + 10)];
    cVar1 = *pcVar2;
    if (cVar1 == -1) goto LAB_00429513;
    if (cVar1 == -3) {
        if (*(unsigned short *)(DAT_0070b1f2 +
                (unsigned int)*(unsigned char *)((unsigned int)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988) * 0x566) == 0) {
LAB_0042949f:
            *(unsigned short *)(iVar3 + 0x18) = 0x8000;
        }
        else {
            *(unsigned short *)(iVar3 + 0x18) = 0x800c;
        }
    }
    else if (cVar1 == -2) {
        if ((*(char *)((unsigned int)DAT_00732e54 + 0x1e5) == '\0') ||
            (*(unsigned short *)(DAT_0070b1f2 +
                (unsigned int)*(unsigned char *)((unsigned int)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988) * 0x566) == 0)) {
            if (*(unsigned short *)(DAT_0070b1f4 +
                    (unsigned int)*(unsigned char *)((unsigned int)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988) * 0x566) == 0)
                goto LAB_0042949f;
            *(unsigned short *)(iVar3 + 0x18) = 0x800d;
        }
        else {
            *(unsigned short *)(iVar3 + 0x18) = 0x800c;
        }
    }
    else if (cVar1 == -7) {
        *(unsigned short *)(iVar3 + 0x18) = 0x800c;
    }
    else {
        if (*(unsigned short *)(DAT_0070b1da +
                (unsigned int)*(unsigned char *)((unsigned int)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988) * 0x566 +
                (unsigned int)*(iVar3 + 0xc) * 2) == 0) goto LAB_0042949f;
        *(unsigned short *)(iVar3 + 0x18) = (unsigned short)(*(iVar3 + 0xc) + 0x8000);
    }
    *(unsigned short *)(iVar3 + 0x1a) = 0;
    *(iVar3 + 0x1e) = 0;
    if (*(short *)(iVar3 + 0x18) != -1) {
        __builtin_memcpy(&pTmp, DAT_006d9d56, 4);
        FUN_00421c70((unsigned int)*(unsigned short *)(pTmp + 0x18),
                     (unsigned int)*(unsigned char *)((unsigned int)DAT_006d9d50 * 0x23 + 0x16 + DAT_006d4988));
        __builtin_memcpy(&pTmp, DAT_006d9d56, 4);
        __builtin_memcpy(pTmp + 0x1f, &DAT_00732a2c, 4);
        __builtin_memcpy(pTmp + 0x23, &DAT_00732a28, 4);
        __builtin_memcpy(pTmp + 0x27, &DAT_00732a24, 4);
    }
LAB_00429513:
    *(unsigned short *)(iVar3 + 0x3a) = 0;
    cVar1 = pcVar2[1];
    if (cVar1 != -1) {
        *(char *)(iVar3 + 0x34) = cVar1;
        FUN_00491ad0();
    }
    *(unsigned short *)((unsigned int)DAT_00732e54 + 0x6a) = 0xffff;
    *(unsigned short *)((unsigned int)DAT_00732e54 + 0x6c) = 0;
    *(unsigned short *)((unsigned int)DAT_00732e54 + 0x6e) = 0xffff;
    __builtin_memcpy(&iTmp, iVar3 + 0xe, 4);
    uTmp = ((unsigned int)iTmp * (param_1 & 7)) >> 3;
    __builtin_memcpy(iVar3 + 0x12, &uTmp, 4);
}
