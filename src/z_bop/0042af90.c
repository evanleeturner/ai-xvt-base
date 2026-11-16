/* SPDX-License-Identifier: MIT */
/* z_bop 0042af90 */
extern int DAT_009ecc34;
extern int DAT_009d7680;
extern char DAT_009e9a5b;
extern int DAT_005233a8;
extern int _DAT_005233b0;
extern int DAT_005233b8;
extern int DAT_005233bc;
extern int DAT_005233c4;
extern int DAT_005233c0;
extern int _DAT_005233b4;

void __stdcall FUN_0042af90(void)
{
    int iVar1;
    unsigned int tmp1;
    unsigned int tmp2;
    unsigned int tmp3;
    unsigned short tmp4;
    unsigned short tmp5;
    unsigned short tmp6;

    iVar1 = DAT_009ecc34 * 0x5bd;
    DAT_009d7680 = 0;
    tmp1 = 0;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1), &tmp1, sizeof(tmp1));
    tmp2 = 0xfffffb00;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1 + 4), &tmp2, sizeof(tmp2));
    tmp3 = 0;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1 + 8), &tmp3, sizeof(tmp3));
    tmp4 = 0;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1 + 14), &tmp4, sizeof(tmp4));
    tmp5 = 0x4000;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1 + 10), &tmp5, sizeof(tmp5));
    tmp6 = 0;
    __builtin_memcpy((void *)((char *)&DAT_009e9a5b + iVar1 + 12), &tmp6, sizeof(tmp6));
    DAT_005233a8 = 0x3f800000;
    _DAT_005233b0 = 0x3f800000;
    DAT_005233b8 = 1;
    DAT_005233bc = 1;
    DAT_005233c4 = 1;
    DAT_005233c0 = 1;
    _DAT_005233b4 = 1;
    return;
}
