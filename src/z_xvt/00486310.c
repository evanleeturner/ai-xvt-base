/* SPDX-License-Identifier: MIT */
/* z_xvt 00486310 */
/* Globals (types inferred) */
extern unsigned int   _DAT_005254e0;
extern unsigned short DAT_006da850[];
extern unsigned char  DAT_0071bb1b[];   /* unaligned int access, indexed * 0x5bd */
extern unsigned char  DAT_0071bb1f[];
extern unsigned char  DAT_0071bb23[];
extern int            DAT_0071ecf4;
extern float          _DAT_005201ec;
extern float          _DAT_005201f0;

extern int DAT_0073255c, DAT_00732558, DAT_00732568, DAT_00732578, DAT_00732584;
extern int DAT_0073253c, DAT_00732530, DAT_00732548, DAT_00732538;
extern int DAT_0070a138, DAT_0070a134, DAT_0070a130, DAT_0070a12c;
extern int DAT_0070a010, DAT_0070a00c, _DAT_0070a008, _DAT_0070a124, _DAT_0070a120;

extern int           DAT_00602678;
extern unsigned int *DAT_0052a470;
extern unsigned int  DAT_006026c8, DAT_006026cc, DAT_006026d8, _DAT_006026dc;
extern unsigned int  DAT_006026e0, DAT_0052a478;
extern int           DAT_006026a0, _DAT_006026c0, DAT_006026a4, _DAT_006026a8;
extern int           DAT_00602690, DAT_006026bc;
extern int           DAT_0052a46c;

/* Prototypes */
int  *__cdecl FUN_0048f4f0(unsigned int param_1);
void  __cdecl FUN_00486000(int *param_1, int param_2);
void  __cdecl FUN_00414fe0(float *param_1, float *param_2);
void  __cdecl FUN_00486080(void *param_1, void *param_2, int param_3, int param_4);
void  __cdecl FUN_00486b80(int param_1, unsigned int *param_2, void *param_3);
void  __cdecl FUN_0048f510(void);

void __cdecl FUN_00486310(int param_1)
{
    int iVar1;
    unsigned int *puVar2;
    int *piVar3;
    int *piVar4;
    int iVar5;
    int iVar6;
    int iVar7;
    unsigned short uStack_dc = 0;
    struct {
        int   local_d0;
        float local_cc;
        float local_c8;
        float local_c4;
        float local_c0;
        float local_bc;
        float local_b8;
        float local_b4;
        float local_b0;
        float local_ac;
        float local_a8;
        float local_a4;
        float local_a0;
        float local_9c;
        float local_98;
        float local_94;
        float local_90;
        float local_8c;
        float local_88;
        float local_84;
        float local_80;
        float local_7c;
        float local_78;
        float local_74;
        float local_70;
        float local_6c;
        unsigned int pad[26];
    } frame;
    int fld;
    int tmp;
    int tmp2;
    unsigned char *p1 = (unsigned char *)param_1;

    __builtin_memcpy(&fld, p1 + 0x1f, 4);
    if (fld == 0) {
        _DAT_005254e0 = 0;
    }
    else {
        _DAT_005254e0 = (unsigned int)*(unsigned char *)(fld + 0x88);
    }

    piVar3 = FUN_0048f4f0(((unsigned int)uStack_dc << 16) |
                          (unsigned int)DAT_006da850[*(unsigned char *)(p1 + 3)]);
    piVar4 = (int *)*piVar3;
    if (piVar3 != piVar4) {
        unsigned char *b3 = (unsigned char *)piVar3;
        iVar7 = 0;
        iVar6 = (int)((unsigned int)(int)piVar3 - (unsigned int)(int)piVar4);
        *piVar3 = (int)((unsigned int)(int)piVar4 + (unsigned int)iVar6);
        __builtin_memcpy(&tmp, b3 + 10, 4);
        if (tmp != 0) {
            iVar5 = 0;
            tmp = (int)((unsigned int)tmp + (unsigned int)iVar6);
            __builtin_memcpy(b3 + 10, &tmp, 4);
            __builtin_memcpy(&tmp2, b3 + 6, 4);
            if (0 < tmp2) {
                do {
                    __builtin_memcpy(&tmp, b3 + 10, 4);
                    piVar4 = (int *)(tmp + iVar7);
                    iVar1 = *piVar4;
                    if (iVar1 != 0) {
                        *piVar4 = (int)((unsigned int)iVar1 + (unsigned int)iVar6);
                        __builtin_memcpy(&tmp, b3 + 10, 4);
                        FUN_00486000(*(int **)(tmp + iVar7), iVar6);
                    }
                    iVar7 = iVar7 + 4;
                    iVar5 = iVar5 + 1;
                    __builtin_memcpy(&tmp2, b3 + 6, 4);
                } while (iVar5 < tmp2);
            }
        }
    }

    {
        unsigned int *z = (unsigned int *)&frame;
        for (iVar6 = 0x34; iVar6 != 0; iVar6 = iVar6 + -1) {
            *z = 0;
            z = z + 1;
        }
    }

    frame.local_d0 = param_1;

    {
        int a, b;
        __builtin_memcpy(&a, p1 + 4, 4);
        __builtin_memcpy(&b, DAT_0071bb1b + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        frame.local_c8 = (float)(int)((unsigned int)a - (unsigned int)b);
        __builtin_memcpy(&a, p1 + 8, 4);
        __builtin_memcpy(&b, DAT_0071bb1f + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        frame.local_c4 = (float)(int)((unsigned int)a - (unsigned int)b);
    }
    frame.local_bc = (float)DAT_0073255c * _DAT_005201ec;
    frame.local_b8 = (float)DAT_00732558 * _DAT_005201ec;
    frame.local_b4 = (float)DAT_00732568 * _DAT_005201ec;
    frame.local_ac = (float)DAT_00732578 * _DAT_005201ec;
    frame.local_b0 = (float)DAT_00732584 * _DAT_005201ec;
    frame.local_a4 = (float)DAT_0073253c * _DAT_005201ec;
    frame.local_a8 = (float)DAT_00732530 * _DAT_005201ec;
    frame.local_9c = (float)DAT_00732548 * _DAT_005201ec;
    frame.local_a0 = (float)DAT_00732538 * _DAT_005201ec;
    {
        int a, b;
        __builtin_memcpy(&a, p1 + 0xc, 4);
        __builtin_memcpy(&b, DAT_0071bb23 + (unsigned int)DAT_0071ecf4 * 0x5bd, 4);
        frame.local_c0 = (float)(int)((unsigned int)a - (unsigned int)b);
    }
    FUN_00414fe0(&frame.local_c8, &frame.local_bc);

    frame.local_b8 = (float)DAT_0070a138 * _DAT_005201ec;
    frame.local_b4 = (float)DAT_0070a134 * _DAT_005201ec;
    frame.local_b0 = (float)DAT_0070a130 * _DAT_005201ec;
    frame.local_bc = (float)DAT_0070a12c * _DAT_005201ec;
    frame.local_ac = (float)DAT_0070a010 * _DAT_005201ec;
    frame.local_a8 = (float)DAT_0070a00c * _DAT_005201ec;
    frame.local_a4 = (float)_DAT_0070a008 * _DAT_005201ec;
    frame.local_a0 = (float)_DAT_0070a124 * _DAT_005201ec;
    frame.local_9c = (float)_DAT_0070a120 * _DAT_005201ec;
    frame.local_98 = -frame.local_c8;
    frame.local_94 = -frame.local_c4;
    frame.local_90 = -frame.local_c0;
    frame.local_8c = frame.local_bc;
    frame.local_88 = frame.local_b0;
    frame.local_84 = frame.local_a4;
    frame.local_80 = frame.local_b8;
    frame.local_7c = frame.local_ac;
    frame.local_78 = frame.local_a0;
    frame.local_74 = frame.local_b4;
    frame.local_70 = frame.local_a8;
    frame.local_6c = frame.local_9c;
    FUN_00414fe0(&frame.local_98, &frame.local_8c);

    DAT_00602678 = 0;
    if (DAT_0052a470 == (unsigned int *)0) {
        DAT_0052a470 = &DAT_006026c8;
        _DAT_006026dc = 8;
        DAT_006026d8 = 8;
        DAT_006026cc = 0x10;
        DAT_006026c8 = 0x100;
        FUN_00486080(&DAT_006026e0, &DAT_0052a478, 8, 8);
    }
    DAT_006026a0 = 0;
    _DAT_006026c0 = 0;
    iVar7 = 0;
    iVar6 = 0;
    DAT_006026a4 = (int)DAT_0052a470;
    _DAT_006026a8 = 0;
    DAT_00602690 = 0;
    DAT_006026bc = 0;

    {
        unsigned char *b3 = (unsigned char *)piVar3;
        __builtin_memcpy(&tmp2, b3 + 6, 4);
        if (0 < tmp2) {
            iVar5 = 0;
            do {
                frame.local_cc = 0.0f;
                __builtin_memcpy(&tmp, b3 + 10, 4);
                puVar2 = *(unsigned int **)(tmp + iVar5);
                if (puVar2[1] == 0x14) {
LAB_0048670e:
                    DAT_0052a46c = DAT_0052a46c + 1;
                    FUN_00486b80((int)piVar3, puVar2, &frame);
                }
                else {
                    iVar7 = iVar7 + 1;
                    __builtin_memcpy(&fld, p1 + 0x1f, 4);
                    if (fld == 0) goto LAB_0048670e;
                    __builtin_memcpy(&iVar1, (unsigned char *)fld + 0xa7, 4);
                    if (iVar1 == 0) goto LAB_0048670e;
                    if (*(char *)(iVar1 + 0x2c9 + iVar7) == '\0') {
                        frame.local_cc =
                            (float)*(unsigned char *)(iVar1 + 0x2f1 + iVar7) * _DAT_005201f0;
                        goto LAB_0048670e;
                    }
                }
                iVar5 = iVar5 + 4;
                iVar6 = iVar6 + 1;
                __builtin_memcpy(&tmp2, b3 + 6, 4);
            } while (iVar6 < tmp2);
        }
    }
    FUN_0048f510();
    return;
}
