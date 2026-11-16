/* SPDX-License-Identifier: MIT */
/* z_xvt 00497c40 */
extern unsigned int DAT_006d4988;
extern unsigned short DAT_006d9d50;
extern unsigned char DAT_006d9d56[4]; /* packed 4-byte pointer at an unaligned address */
extern short DAT_006da9f0;
extern unsigned int DAT_006db580;
extern short DAT_00732e40;
extern unsigned int DAT_00732e54;

void __cdecl FUN_00497f00(unsigned int param_1);
void __cdecl FUN_0042a060(void);
void __cdecl FUN_00418cd0(int param_1, int param_2, int param_3);
unsigned int __cdecl FUN_0042b250(int param_1);

void __cdecl FUN_00497c40(short param_1)
{
    unsigned char *psVar1;
    unsigned short uVar2;
    unsigned int uVar3;
    unsigned short uVar4;
    unsigned char *iVar5;
    unsigned char *iVar6;
    unsigned short local_1;
    int local_2;
    int local_3;
    int local_4;
    int local_5;
    int local_6;
    int local_7;
    short local_8;
    short local_9;
    unsigned short local_10;

    uVar3 = (unsigned int)DAT_006d9d50;
    __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
    if (*(char *)(iVar6 + 0x34) == '\x17') {
        FUN_0042a060();
    }
    else {
        __builtin_memcpy(&local_1, iVar6 + 0x18, 2);
        __builtin_memcpy(&local_2, (unsigned char *)((unsigned int)local_1 * 0x23u + 0x1fu + DAT_006d4988), 4);
        if (local_2 == 0) {
            FUN_0042a060();
        }
        else {
            FUN_00497f00((unsigned int)local_1);
        }
    }
    iVar5 = (unsigned char *)(DAT_006d4988 + uVar3 * 0x23u);
    __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
    __builtin_memcpy(&local_2, iVar6 + 0x1f, 4);
    __builtin_memcpy(&local_3, iVar5 + 4, 4);
    __builtin_memcpy(&local_4, iVar6 + 0x23, 4);
    __builtin_memcpy(&local_5, iVar5 + 8, 4);
    __builtin_memcpy(&local_6, iVar6 + 0x27, 4);
    __builtin_memcpy(&local_7, iVar5 + 0xc, 4);
    FUN_00418cd0((int)((unsigned int)local_2 - (unsigned int)local_3),
                 (int)((unsigned int)local_4 - (unsigned int)local_5),
                 (int)((unsigned int)local_6 - (unsigned int)local_7));
    __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
    local_8 = (short)((unsigned int)(int)DAT_006da9f0 + (unsigned int)(int)param_1);
    __builtin_memcpy(iVar6 + 0x32, &local_8, 2);
    if (*(char *)((unsigned char *)DAT_00732e54 + 0xa4) == '\x02') {
        __builtin_memcpy(&local_8, (unsigned char *)(DAT_006d4988 + uVar3 * 0x23u + 0x10u), 2);
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&local_9, iVar6 + 0x32, 2);
        uVar2 = (unsigned short)((unsigned int)(int)local_8 - (unsigned int)(int)local_9);
        if (0x8000 < uVar2) {
            uVar2 = (unsigned short)(0u - (unsigned int)uVar2);
        }
        if ((uVar2 < 0x2000) && (0xffffu < DAT_006db580)) goto LAB_00497e56;
        uVar3 = FUN_0042b250((int)DAT_00732e54);
        uVar2 = (unsigned short)(uVar3 >> 1);
        psVar1 = (unsigned char *)((unsigned int)DAT_006d9d50 * 0x23u + 0x10u + DAT_006d4988);
        __builtin_memcpy(&local_8, psVar1, 2);
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&local_9, iVar6 + 0x32, 2);
        uVar4 = (unsigned short)((unsigned int)(int)local_8 - (unsigned int)(int)local_9);
        if (0x7fff < uVar4) {
            uVar4 = (unsigned short)(0u - (unsigned int)uVar4);
        }
        if (0x300 < uVar4) goto LAB_00497e3c;
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&local_8, iVar6 + 0x32, 2);
        __builtin_memcpy(psVar1, &local_8, 2);
    }
    else {
        uVar3 = FUN_0042b250((int)DAT_00732e54);
        uVar2 = (unsigned short)(uVar3 >> 1);
        psVar1 = (unsigned char *)((unsigned int)DAT_006d9d50 * 0x23u + 0x10u + DAT_006d4988);
        __builtin_memcpy(&local_8, psVar1, 2);
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&local_9, iVar6 + 0x32, 2);
        uVar4 = (unsigned short)((unsigned int)(int)local_8 - (unsigned int)(int)local_9);
        if (0x7fff < uVar4) {
            uVar4 = (unsigned short)(0u - (unsigned int)uVar4);
        }
        if (0x300 < uVar4) {
LAB_00497e3c:
            *(unsigned char *)((unsigned char *)DAT_00732e54 + 0xab) = 2;
            local_10 = (unsigned short)((unsigned int)(uVar2 & 0x7fff) + 0x8000u);
            __builtin_memcpy((unsigned char *)DAT_00732e54 + 0xac, &local_10, 2);
            goto LAB_00497e56;
        }
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&local_8, iVar6 + 0x32, 2);
        __builtin_memcpy(psVar1, &local_8, 2);
    }
    __builtin_memcpy(&iVar5, (unsigned char *)((unsigned int)DAT_006d9d50 * 0x23u + 0x1fu + DAT_006d4988), 4);
    *(unsigned char *)(iVar5 + 0x90) = 1;
    __builtin_memcpy(&iVar5, (unsigned char *)((unsigned int)DAT_006d9d50 * 0x23u + 0x1fu + DAT_006d4988), 4);
    *(unsigned char *)(iVar5 + 0x89) = *(unsigned char *)(iVar5 + 0x90);
    *(unsigned char *)((unsigned char *)DAT_00732e54 + 0xab) = 3;
LAB_00497e56:
    if (*(char *)((unsigned char *)DAT_00732e54 + 0xa4) == '\x03') {
        *(unsigned char *)((unsigned char *)DAT_00732e54 + 0xa4) = 0;
    }
    __builtin_memcpy(&local_8, (unsigned char *)DAT_00732e54 + 0xd, 2);
    if (local_8 != DAT_00732e40) {
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(iVar6 + 0x2e, &DAT_00732e40, 2);
        local_10 = 0xffff;
        __builtin_memcpy((unsigned char *)DAT_00732e54 + 0x9e, &local_10, 2);
        __builtin_memcpy(&local_1, (unsigned char *)DAT_00732e54 + 0xd, 2);
        __builtin_memcpy(&iVar6, DAT_006d9d56, 4);
        __builtin_memcpy(&uVar4, iVar6 + 0x2e, 2);
        if (local_1 < uVar4) {
            *(unsigned char *)((unsigned char *)DAT_00732e54 + 0x9c) = 2;
        }
        else {
            *(unsigned char *)((unsigned char *)DAT_00732e54 + 0x9c) = 1;
        }
        local_10 = 0xffff;
        __builtin_memcpy((unsigned char *)DAT_00732e54 + 0xbd, &local_10, 2);
        *(unsigned char *)((unsigned char *)DAT_00732e54 + 0x96) = 0;
        *(unsigned char *)((unsigned char *)DAT_00732e54 + 0x97) = 0;
        *(unsigned char *)((unsigned char *)DAT_00732e54 + 0x9d) = 0;
    }
    return;
}
