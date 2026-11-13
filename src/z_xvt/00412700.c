/* SPDX-License-Identifier: MIT */
/* z_xvt 00412700 */
extern unsigned char *DAT_006d4988;
extern int DAT_0071ecf4;
extern unsigned int FUN_004784d0(void);
extern void FUN_0040a1a0(unsigned int, unsigned int, int);

void __cdecl FUN_00412700(unsigned int param_1, unsigned char param_2)
{
    unsigned int uVar1;
    int iVar2;
    unsigned int idx;
    void *ptr;
    unsigned short tmp;

    idx = param_1 * 0x23;
    *(unsigned char *)(DAT_006d4988 + 3 + idx) = param_2;
    *(unsigned char *)(DAT_006d4988 + 2 + idx) = 0xd;
    __builtin_memcpy(&ptr, DAT_006d4988 + 0x1f + idx, 4);
    *(unsigned char *)ptr = 5;
    *(unsigned char *)(DAT_006d4988 + 0x19 + idx) = 2;
    *(unsigned char *)((unsigned char *)ptr + 1) = 0;
    tmp = 0;
    __builtin_memcpy((unsigned char *)ptr + 0x79, &tmp, 2);
    __builtin_memcpy((unsigned char *)ptr + 0x81, &tmp, 2);
    __builtin_memcpy((unsigned char *)ptr + 0x7f, &tmp, 2);
    *(unsigned short *)(DAT_006d4988 + 0x14 + idx) = 0;
    __builtin_memcpy((unsigned char *)ptr + 0x77, &tmp, 2);
    *(unsigned char *)((unsigned char *)ptr + 0x90) = 1;
    iVar2 = DAT_0071ecf4;
    uVar1 = FUN_004784d0();
    FUN_0040a1a0((uVar1 & 3) + 0x16, param_1, iVar2);
    return;
}
