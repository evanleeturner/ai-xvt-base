/* SPDX-License-Identifier: MIT */
/* z_xvt 004bff10 */
extern int DAT_0081b848;
extern int DAT_0081b6f4;
extern int DAT_0081bcb0;
extern int DAT_0081bcd0;
extern int DAT_0081bcf0;

extern int __cdecl FUN_004dd000(int *param_1, int *param_2);
extern int __cdecl FUN_004ddc70(int *param_1, int *param_2);
extern int __cdecl FUN_004dd7d0(int *param_1, int param_2, int param_3);
extern int __cdecl FUN_004dd130(int *param_1, int param_2, int param_3, int param_4);
extern int __cdecl FUN_004db570(void);
extern int __cdecl FUN_004db590(void);
extern int __cdecl FUN_004d5180(int *param_1);
extern int __cdecl FUN_004d51b0(int *param_1);
extern int __cdecl __strcmpi(const char *param_1, const char *param_2);
extern int __cdecl FUN_004daac0(int param_1, char *param_2, int *param_3, int param_4, int param_5, unsigned int param_6);
extern int __cdecl FUN_004dd030(int *param_1, int param_2, int param_3);

int __cdecl FUN_004bff10(int *param_1, int param_2)
{
    int iStack_2c;
    int iStack_28 = 0;
    int iStack_20;
    int iStack_1c;
    int iStack_18;
    int iStack_14;
    int auStack_10[4];
    int iVar2;
    int iVar4;
    int iVar5;
    int iVar6;
    unsigned int uVar7;
    char *pcVar3;

    if (DAT_0081b848 == 0) {
        return 0;
    }
    FUN_004dd000(&iStack_2c, param_1);
    FUN_004ddc70(&iStack_14, &iStack_18);
    iStack_20 = iStack_28 + 0xe;
    (void)iStack_20;
    iVar4 = 0;
    iStack_1c = *(int *)(DAT_0081b848 + 4);
    if ((DAT_0081b6f4 != 0) && (param_2 < param_2 + 0xd)) {
        iVar5 = param_2 * 0xe;
        iVar6 = param_2;
        do {
            if (iStack_1c <= iVar6) {
                return iVar4;
            }
            iVar2 = FUN_004dd7d0(&iStack_2c, iStack_14, iStack_18);
            if (iVar2 != 0) {
                FUN_004dd130(&iStack_2c, 0, 0, DAT_0081bcb0);
                if (FUN_004db570() != 0 || FUN_004db590() != 0) {
                    iVar4 = iVar6 + 1;
                }
            }
            FUN_004d5180(auStack_10);
            FUN_004d51b0(&iStack_2c);
            iVar2 = __strcmpi((char *)(DAT_0081b6f4 + iVar5), (char *)&DAT_0081bcf0);
            uVar7 = DAT_0081bcd0;
            if (iVar2 != 0) {
                uVar7 = 0xffff;
            }
            iVar6 = iVar6 + 1;
            pcVar3 = (char *)(DAT_0081b6f4 + iVar5);
            iVar5 = iVar5 + 0xe;
            FUN_004daac0(0xc, pcVar3, &iStack_2c, 0, 1, uVar7);
            FUN_004d51b0(auStack_10);
            FUN_004dd030(&iStack_2c, 0, 0xf);
        } while (iVar6 < param_2 + 0xd);
    }
    return iVar4;
}
