/* SPDX-License-Identifier: MIT */
/* z_xvt 00429a20 */
int __cdecl FUN_0042b150(int param_1);
void __cdecl FUN_00421c70(unsigned int param_1, int param_2);

extern int DAT_006d4988;
extern int DAT_006d499c;
extern unsigned short DAT_006d9d50;
extern int DAT_00732a24;
extern int DAT_00732a28;
extern int DAT_00732a2c;

unsigned char __cdecl FUN_00429a20(unsigned int param_1)
{
    unsigned char cVar1;
    int iVar2;
    int iVar3;
    int iVar4;
    int iVar5;
    unsigned int local_8;
    unsigned char *pEntry;
    int iEntry1f;
    int iTmp;
    unsigned short usA;
    unsigned short usB;

    if (param_1 == 0xffff) {
        return 0;
    }
    iVar3 = (int)(param_1 * 0x23u + (unsigned int)DAT_006d4988);
    pEntry = (unsigned char *)iVar3;
    __builtin_memcpy(&iEntry1f, pEntry + 0x1f, 4);
    if (iEntry1f == 0) {
        return (unsigned char)(1 - (*(char *)(pEntry + 3) == '\0'));
    }
    if (*(char *)(pEntry + 3) == '\0') {
        return 0;
    }
    __builtin_memcpy(&iVar3, (unsigned char *)iEntry1f + 0xa7, 4);
    if (iVar3 != 0) {
        iVar2 = FUN_0042b150(iVar3);
        if ((*(char *)(iVar2 + 0x34) == '\x15') && (*(char *)(iVar2 + 0x35) != '\0')) {
            return 0;
        }
        cVar1 = *(char *)(iVar3 + 7);
        if ((cVar1 != '\x01') && ((cVar1 != '\x03' && (cVar1 != '\x04')))) {
            if ((*(char *)(iVar3 + 0x233) == '\x03') && (*(char *)(iVar3 + 0x237) != '\0')) {
                local_8 = (unsigned int)DAT_006d9d50;
                FUN_00421c70(param_1, 0);
                iVar5 = DAT_00732a2c;
                iVar4 = DAT_00732a28;
                iVar2 = DAT_00732a24;
                FUN_00421c70(local_8, 0);
                iVar5 = (int)((unsigned int)iVar5 - (unsigned int)DAT_00732a2c);
                iVar4 = (int)((unsigned int)iVar4 - (unsigned int)DAT_00732a28);
                iVar2 = (int)((unsigned int)iVar2 - (unsigned int)DAT_00732a24);
                if (iVar5 < 0) {
                    iVar5 = (int)(0u - (unsigned int)iVar5);
                }
                if (iVar4 < 0) {
                    iVar4 = (int)(0u - (unsigned int)iVar4);
                }
                if (iVar2 < 0) {
                    iVar2 = (int)(0u - (unsigned int)iVar2);
                }
                if (iVar4 < iVar5) {
                    DAT_006d499c = (int)((unsigned int)(iVar4 >> 1) + (unsigned int)iVar5);
                }
                else {
                    DAT_006d499c = (int)((unsigned int)(iVar5 >> 1) + (unsigned int)iVar4);
                }
                if (iVar2 < DAT_006d499c) {
                    iVar2 = iVar2 >> 1;
                }
                else {
                    DAT_006d499c = DAT_006d499c >> 1;
                }
                DAT_006d499c = (int)((unsigned int)DAT_006d499c + (unsigned int)iVar2);
                cVar1 = *(char *)((unsigned int)DAT_006d9d50 * 0x23u + 2u + (unsigned int)DAT_006d4988);
                if (((cVar1 == '\0') || (cVar1 == '\x01')) || (cVar1 == '\x02')) {
                    if (0x8000 < DAT_006d499c) {
                        return 0;
                    }
                }
                else if (0x10000 < DAT_006d499c) {
                    return 0;
                }
            }
            iVar2 = (int)((unsigned int)DAT_006d4988 + param_1 * 0x23u);
            __builtin_memcpy(&iTmp, (unsigned char *)iVar2 + 0x1b, 4);
            if (iTmp != -1) {
                return 1;
            }
            __builtin_memcpy(&iTmp, (unsigned char *)iVar2 + 0x1f, 4);
            __builtin_memcpy(&iTmp, (unsigned char *)iTmp + 0xa7, 4);
            __builtin_memcpy(&usA, (unsigned char *)iTmp + 199, 2);
            __builtin_memcpy(&usB, (unsigned char *)iVar3 + 0xc3, 2);
            return (unsigned char)(1 - (usA < usB));
        }
        return 0;
    }
    return 1;
}
