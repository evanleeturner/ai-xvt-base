/* SPDX-License-Identifier: MIT */
/* z_xvt 00490be0 */
int __cdecl _strncmp(const char *, const char *, unsigned int);
int __cdecl FUN_00417c20(int *param_1, char *param_2);

extern int *DAT_00658654;
extern char DAT_0065865c[];
extern unsigned char DAT_006587a0[];
extern int DAT_006a8814;
extern int DAT_006a8818;
extern int DAT_006a881c;
extern int DAT_006a860c[];
extern int DAT_006a8610[];
extern int DAT_006a8614[];

int __cdecl FUN_00490be0(char *param_1, int param_2, int param_3, char *param_4,
                         int param_5, int param_6)
{
    int iVar1;
    int *piVar2;
    int iVar3;
    char *_Str1;
    int iVar4;
    int iVar5;
    int *unaff_EDI;
    int *piVar7;
    int *piVar8;
    int iStack_c;
    char *pcStack_8;
    unsigned int uStack_4;

    if (DAT_00658654 == (int *)0x0) {
        return 0;
    }
    if (*param_1 == '\0') {
        return 0;
    }
    iVar3 = 0;
    pcStack_8 = (char *)DAT_0065865c;
    iVar4 = (int)((unsigned int)DAT_006a8814 - 1u);
    while (-1 < iVar4) {
        iVar5 = iVar4 >> 1;
        _Str1 = pcStack_8 + (int)((unsigned int)iVar5 * 0x145u);
        iVar1 = _strncmp(_Str1, param_1, 0x40);
        if (iVar1 == 0) {
            iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar3);
            goto LAB_00490c76;
        }
        if (iVar4 < 1) break;
        if (iVar1 < 0) {
            iVar3 = (int)((unsigned int)iVar5 + 1u + (unsigned int)iVar3);
            iVar4 = (int)((unsigned int)iVar4 - ((unsigned int)iVar5 + 1u));
            pcStack_8 = _Str1 + 0x145;
        } else {
            iVar4 = (int)((unsigned int)iVar5 - 1u);
        }
    }
    iVar5 = -1;
LAB_00490c76:
    if (iVar5 == -1) {
        return 0;
    }
    if (DAT_006a8818 == 8) {
        iVar4 = 0;
        do {
            if (DAT_006a860c[iVar4 * 3] != -1) {
                piVar2 = (int *)DAT_006a8614[iVar4 * 3];
                iVar3 = (*(int (__stdcall **)(int *, unsigned int *))(*piVar2 + 0x24))(piVar2, &uStack_4);
                if (iVar3 != 0 || (uStack_4 & 5) == 0) {
                    piVar2 = (int *)DAT_006a8614[iVar4 * 3];
                    (*(int (__stdcall **)(int *))(*piVar2 + 8))(piVar2);
                    DAT_006a860c[iVar4 * 3] = -1;
                    DAT_006a8614[iVar4 * 3] = 0;
                    DAT_006a8818 = (int)((unsigned int)DAT_006a8818 - 1u);
                    break;
                }
            }
            iVar4 = (int)((unsigned int)iVar4 + 1u);
        } while (iVar4 < 8);
        if (iVar4 == 8) {
            iVar4 = 8;
            iVar3 = 0;
            pcStack_8 = param_4;
            do {
                if ((int)(unsigned int)DAT_006587a0[(int)((unsigned int)DAT_006a860c[iVar3 * 3] * 0x145u)] < (int)pcStack_8) {
                    iVar4 = iVar3;
                    pcStack_8 = (char *)(unsigned int)DAT_006587a0[(int)((unsigned int)DAT_006a860c[iVar3 * 3] * 0x145u)];
                }
                iVar3 = (int)((unsigned int)iVar3 + 1u);
            } while (iVar3 < 8);
            if (iVar4 == 8) {
                if (param_2 != 0) {
                    iVar4 = 0;
                    do {
                        if (DAT_006a860c[iVar4 * 3] == iVar5) {
                            piVar2 = (int *)DAT_006a8614[iVar4 * 3];
                            (*(int (__stdcall **)(int *, int))(*piVar2 + 0x34))(piVar2, 0);
                            DAT_006a8610[iVar4 * 3] = DAT_006a881c;
                            DAT_006a881c = (int)((unsigned int)DAT_006a881c + 1u);
                            return 1;
                        }
                        iVar4 = (int)((unsigned int)iVar4 + 1u);
                    } while (iVar4 < 8);
                }
                return 0;
            }
            piVar2 = (int *)DAT_006a8614[iVar4 * 3];
            if (DAT_006a860c[iVar4 * 3] == iVar5) {
                (*(int (__stdcall **)(int *, int))(*piVar2 + 0x34))(piVar2, 0);
                DAT_006a8610[iVar4 * 3] = DAT_006a881c;
                DAT_006a881c = (int)((unsigned int)DAT_006a881c + 1u);
                return 1;
            }
            (*(int (__stdcall **)(int *))(*piVar2 + 0x48))(piVar2);
            piVar2 = (int *)DAT_006a8614[iVar4 * 3];
            (*(int (__stdcall **)(int *))(*piVar2 + 8))(piVar2);
            DAT_006a860c[iVar4 * 3] = -1;
            DAT_006a8614[iVar4 * 3] = 0;
            DAT_006a8818 = (int)((unsigned int)DAT_006a8818 - 1u);
        }
    } else {
        iVar4 = 0;
        do {
            if (DAT_006a860c[iVar4 * 3] == -1) break;
            iVar4 = (int)((unsigned int)iVar4 + 1u);
        } while (iVar4 < 8);
    }
    piVar7 = DAT_00658654;
    __builtin_memcpy(&iVar3, DAT_0065865c + (int)((unsigned int)iVar5 * 0x145u) + 0x140, sizeof(iVar3));
    (*(int (__stdcall **)(int *, int, int *))(*piVar7 + 0x14))(piVar7, iVar3, &iStack_c);
    unaff_EDI = (int *)iStack_c;
    if (unaff_EDI == (int *)0x0) {
        return 0;
    }
    piVar8 = unaff_EDI;
    (*(int (__stdcall **)(int *, int))(*unaff_EDI + 0x34))(unaff_EDI, 0);
    if (param_5 < 0x80) {
        if (param_5 < 0) {
            param_5 = 0;
        }
    } else {
        param_5 = 0x7f;
    }
    (*(int (__stdcall **)(int *, int))(*piVar8 + 0x3c))(piVar8, (int)(((unsigned int)param_5 * 5u - 0x27bu) * 400u) / 0x7f);
    uStack_4 = (unsigned int)param_6;
    if (0x7f < (int)uStack_4) {
        uStack_4 = 0x7f;
    }
    if ((int)uStack_4 < 0) {
        uStack_4 = 0;
    }
    (*(int (__stdcall **)(int *, int))(*piVar8 + 0x40))(piVar8, (int)((uStack_4 * 5u - 0x13bu) * 400u) / 0x3f);
    iVar3 = (*(int (__stdcall **)(int *, int, int, int))(*piVar8 + 0x30))(piVar8, 0, 0, param_3 == 1);
    if (iVar3 == -0x7787ff6a) {
        __builtin_memcpy(&iVar1, DAT_0065865c + (int)((unsigned int)iVar5 * 0x145u) + 0x140, sizeof(iVar1));
        iVar3 = FUN_00417c20((int *)iVar1, DAT_0065865c + (int)((unsigned int)iVar5 * 0x145u) + 0x40);
        if (iVar3 == 1) {
            (*(int (__stdcall **)(int *, int))(*piVar8 + 0x34))(piVar8, 0);
            iVar3 = (*(int (__stdcall **)(int *, int, int, int))(*piVar8 + 0x30))(piVar8, 0, 0, param_3 == 1);
            if (iVar3 != 0) {
                return 0;
            }
            DAT_006a860c[iVar4 * 3] = iVar5;
            DAT_006a8614[iVar4 * 3] = (int)piVar8;
            DAT_006a8610[iVar4 * 3] = DAT_006a881c;
            DAT_006a881c = (int)((unsigned int)DAT_006a881c + 1u);
            DAT_006a8818 = (int)((unsigned int)DAT_006a8818 + 1u);
            return 0;
        }
    } else if (iVar3 == 0) {
        DAT_006a860c[iVar4 * 3] = iVar5;
        DAT_006a8614[iVar4 * 3] = (int)piVar8;
        DAT_006a8610[iVar4 * 3] = DAT_006a881c;
        DAT_006a881c = (int)((unsigned int)DAT_006a881c + 1u);
        DAT_006a8818 = (int)((unsigned int)DAT_006a8818 + 1u);
        iVar3 = 1;
    }
    return iVar3;
}
