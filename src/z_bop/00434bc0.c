/* SPDX-License-Identifier: MIT */
/* z_bop 00434bc0 */
extern int DAT_009a1fe8;
extern char DAT_009d8e25[];
extern char DAT_009d8e29[];
extern char DAT_009d8c88[];
extern char DAT_009fd458[];
extern char DAT_009d6954;
extern char DAT_009e967c[];
extern char DAT_009e96dd[];
extern char DAT_009e96e1[];
extern char DAT_009e967a[];
extern char DAT_009e97c5[];
extern char DAT_009e98d3[];
extern char DAT_009e9889[];
extern char DAT_009e9765[];
extern char DAT_009e98c7[];
extern char DAT_009e98ef[];
extern char DAT_009e9857[];
extern char DAT_009e9919[];
extern char DAT_009e9705[];
extern char DAT_009e98bb[];
extern char DAT_009e98df[];
extern char DAT_009e9825[];
extern char DAT_009e9909[];
extern char DAT_009e98ff[];
extern int DAT_009ecc34;
extern short DAT_00a08292;
extern short DAT_00a07be0;

int __cdecl FUN_00435a40(unsigned int);
int __cdecl FUN_00435640(int, int, int, int, int, unsigned int);
void __cdecl FUN_0042fb70(int, unsigned int, int, int, unsigned int, unsigned int);
void __cdecl FUN_00450650(int, int);
void __cdecl FUN_00430200(int, int, unsigned int, int);

void __cdecl
FUN_00434bc0(unsigned int param_1, int param_2, int param_3, int param_4, int param_5, int param_6,
             unsigned int param_7, int param_8)
{
    char cVar1;
    int iVar2;
    unsigned int uVar3;
    unsigned short uVar4;
    int iVar5;
    char *pcVar6;
    int iVar7;
    char *pcVar8;
    unsigned short uStack_e;
    unsigned short local_8;
    unsigned short us;
    unsigned short idx;
    int iv;
    char *p;

    uStack_e = 0;
    local_8 = 1;
    uVar4 = (unsigned short)*(unsigned char *)((((param_1 & 0xffff) * 8 - (param_1 & 0xffff)) * 5 + 0x16) + DAT_009a1fe8);
    uVar3 = (unsigned int)uVar4;
    iVar5 = (int)(uVar3 * 0x566);
    pcVar6 = DAT_009d8e25 + iVar5;
    iVar7 = 8;
    iVar2 = (int)((unsigned int)param_4 * 0x5bd);
    __builtin_memcpy(&idx, DAT_009e967c + iVar2, 2);
    pcVar8 = DAT_009d8e29 + (unsigned int)idx + iVar5;
    do {
        if ((*pcVar6 == '\0') && (*pcVar8 == '\x01')) {
            cVar1 = pcVar6[1];
            if (cVar1 == '\0') {
LAB_00434c77:
                uStack_e = 0x6000;
            }
            else if (cVar1 == '\x02') {
                uStack_e = 0xf000;
            }
            else if (cVar1 == '\n') goto LAB_00434c77;
        }
        pcVar6 = pcVar6 + 0x4e;
        pcVar8 = pcVar8 + 0x4e;
        iVar7 = iVar7 + -1;
        if (iVar7 == 0) {
            __builtin_memcpy(&idx, DAT_009e967c + iVar2, 2);
            if (DAT_009fd458[(unsigned int)idx * 0x1e5 + (unsigned int)(unsigned char)DAT_009d8c88[iVar5]] == '\0') {
                iVar5 = FUN_00435a40(param_1 & 0xffff);
                if (param_3 == 1) {
                    p = DAT_009e97c5 + uVar3 * 2 + iVar2;
                    __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    if (param_5 == -1) {
                        p = DAT_009e98d3 + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    else {
                        p = DAT_009e9889 + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    iVar5 = iVar5 / 10;
                    local_8 = 10;
                }
                else if (param_3 == 2) {
                    p = DAT_009e9765 + uVar3 * 2 + iVar2;
                    __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    if (param_5 == -1) {
                        p = DAT_009e98c7 + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    else {
                        p = DAT_009e98ef + (int)((unsigned int)param_5 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                        p = DAT_009e9857 + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                        p = DAT_009e9919 + (int)((unsigned int)param_4 * 2) + (int)((unsigned int)param_5 * 0x5bd);
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    local_8 = 6;
                    iVar5 = iVar5 / 2;
                }
                else if (param_3 == 3) {
                    p = DAT_009e9705 + uVar3 * 2 + iVar2;
                    __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    if (param_5 == -1) {
                        p = DAT_009e98bb + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    else {
                        p = DAT_009e98df + (int)((unsigned int)param_5 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                        p = DAT_009e9825 + (int)((unsigned int)param_6 * 2) + iVar2;
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                        p = DAT_009e9909 + (int)((unsigned int)param_4 * 2) + (int)((unsigned int)param_5 * 0x5bd);
                        __builtin_memcpy(&us, p, 2); us = (unsigned short)(us + 1); __builtin_memcpy(p, &us, 2);
                    }
                    local_8 = 1;
                }
                if (DAT_009d6954 == '\x02') {
                    iVar5 = (int)((unsigned int)iVar5 * 2);
                }
                else if (DAT_009d6954 == '\0') {
                    iVar5 = iVar5 / 2;
                }
                __builtin_memcpy(&idx, DAT_009e967c + iVar2, 2);
                iVar7 = FUN_00435640(2, (int)(((unsigned int)uStack_e << 16) | (unsigned int)uVar4), param_4,
                                     local_8, param_2, (unsigned int)idx);
                if (-1 < iVar7) {
                    __builtin_memcpy(&iv, DAT_009e96dd + iVar2, 4);
                    iv = (int)((unsigned int)iv + (unsigned int)iVar5);
                    __builtin_memcpy(DAT_009e96dd + iVar2, &iv, 4);
                }
                if ((param_3 == 3) && (DAT_009ecc34 == param_4)) {
                    FUN_0042fb70(DAT_009ecc34, 0xffffffff, 0x15, -1, 0xffffffff, (unsigned int)uStack_e);
                    return;
                }
            }
            else if ((param_3 == 2) || (param_3 == 3)) {
                iVar5 = FUN_00435a40(param_1 & 0xffff);
                __builtin_memcpy(&iv, DAT_009e96dd + iVar2, 4);
                iv = (int)((unsigned int)iv - (unsigned int)iVar5);
                __builtin_memcpy(DAT_009e96dd + iVar2, &iv, 4);
                __builtin_memcpy(&us, DAT_009e98ff + iVar2, 2);
                us = (unsigned short)(us + 1);
                __builtin_memcpy(DAT_009e98ff + iVar2, &us, 2);
                __builtin_memcpy(&iv, DAT_009e96e1 + iVar2, 4);
                iv = (int)((unsigned int)iv + (unsigned int)-500);
                __builtin_memcpy(DAT_009e96e1 + iVar2, &iv, 4);
                __builtin_memcpy(&DAT_00a08292, DAT_009e967a + iVar2, 2);
                FUN_00450650(0x119, param_4);
                if (iVar5 != 0) {
                    DAT_00a07be0 = (short)iVar5;
                    FUN_00450650(200, param_4);
                }
                if (DAT_009ecc34 == param_4) {
                    FUN_00430200(2, 0x39, param_1 & 0xffff, 0xffff);
                }
            }
            return;
        }
    } while (1);
}
