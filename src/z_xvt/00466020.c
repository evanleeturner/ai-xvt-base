/* SPDX-License-Identifier: MIT */
/* z_xvt 00466020 */
extern int DAT_0052ae18;
extern char DAT_00732e8f;
extern int _DAT_005f2608;
extern int DAT_006a8850;
extern unsigned char DAT_006a884a;   /* unaligned: read via memcpy */
extern int DAT_005f1d30;
extern unsigned int DAT_005f1d78;
extern int DAT_005254ec;
extern float _DAT_005f260c;
extern float _DAT_005f2610;
extern unsigned int _DAT_005f2614;
extern unsigned int DAT_005f2618;
extern unsigned int DAT_005f261c;
extern int DAT_005f2620;
extern int DAT_005f2624;
extern unsigned int DAT_00527d70[];
extern float DAT_005f1d44;
extern float _DAT_006da4fc;
extern float DAT_0070acc8;
extern float _DAT_006db7bc;
extern float DAT_00525518;
extern unsigned int DAT_005f1d34;
extern unsigned int DAT_005f1d3c;
extern float _DAT_005f1d4c;
extern float _DAT_005f1d64;
extern float _DAT_005f1d60;
extern float DAT_0052a350[];
extern unsigned int DAT_005f1d84;
extern unsigned char DAT_005f262c;
extern unsigned int DAT_005f258c;

void FUN_00414520(void);
void FUN_0045f390(void);
void FUN_004a6a80(void);
void FUN_004a6f90(void);
long long __ftol(void);   /* x87 ST0 input not visible in decompile */
void FUN_00467570(unsigned int, int, float);

#define F30(off, T) (*(T *)(DAT_005f1d30 + (off)))

void __stdcall FUN_00466020(void)
{
    float fVar1;
    int iVar2;
    int iVar3;
    unsigned int uVar4;
    int *piVar5;
    unsigned int uVar6;
    int *piVar7;
    int iVar8;
    long long lVar9;
    float fStack_28;
    int local_24;
    int local_20;
    struct {
        unsigned char local_1c[8];
        float fStack_14;
        int iStack_10;
    } loc;
    int base;

    FUN_00414520();
    if (DAT_0052ae18 != 0) {
        FUN_0045f390();
        return;
    }
    if (DAT_00732e8f == '\0') {
        FUN_004a6a80();
    }
    _DAT_005f2608 = 0;
    local_20 = 0;
    if (0 < DAT_006a8850) {
        local_24 = 0;
        do {
            __builtin_memcpy(&base, &DAT_006a884a, 4);
            DAT_005f1d30 = (int)((unsigned int)base + (unsigned int)local_24);
            DAT_005f1d78 = F30(0x40, unsigned int);
            F30(0x38, void *) = loc.local_1c;
            iVar8 = 0;
            piVar7 = (int *)(DAT_005f1d30 + 4);
            iVar2 = *(int *)(*(int *)(DAT_005f1d30 + 4) + 0xa8);
            uVar6 = *(unsigned int *)(iVar2 + 0x14);
            uVar4 = *(unsigned int *)(iVar2 + 0x10);
            if ((DAT_005254ec != 0) && (uVar6 * uVar4 - *(unsigned int *)(iVar2 + 8) == 0)) {
                lVar9 = __ftol();
                iVar2 = (int)lVar9;
                for (; ((0x100 < iVar2 && (uVar4 != 8)) && (uVar6 != 8)); uVar6 = (unsigned int)((int)uVar6 >> 1)) {
                    iVar2 = iVar2 >> 2;
                    iVar3 = (int)(uVar6 * uVar4);
                    uVar4 = (unsigned int)((int)uVar4 >> 1);
                    iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar3);
                }
            }
            _DAT_005f260c = (float)(int)uVar4;
            _DAT_005f2610 = (float)(int)uVar6;
            _DAT_005f2614 = DAT_00527d70[(int)(uVar4 & 0xfffffff3) >> 2];
            DAT_005f2624 = (int)(uVar6 * uVar4 - 1);
            DAT_005f2618 = DAT_00527d70[(int)(uVar6 & 0xfffffff3) >> 2];
            DAT_005f261c = *(unsigned int *)(*piVar7 + 0xb0);
            DAT_005f2620 = (int)((unsigned int)*(int *)(*piVar7 + 0xac) + (unsigned int)iVar8);
            _DAT_005f2608 = *piVar7;
            fStack_28 = (float)DAT_005f1d78 * F30(0x2c, float) + F30(0x30, float);
            DAT_005f1d44 = (_DAT_006da4fc + (float)DAT_005f1d78) * DAT_0070acc8 + _DAT_006db7bc * DAT_00525518;
            if (DAT_005f1d78 < F30(0x44, unsigned int)) {
                iVar2 = 0;
                do {
                    piVar7 = *(int **)(F30(0x68, int) + iVar2);
                    if (piVar7 != (int *)0) {
                        uVar6 = DAT_005f1d34 & DAT_005f1d78;
                        if (uVar6 == 0) {
                            _DAT_005f1d4c = 0.0f;
                            _DAT_005f1d64 = 0.0f;
                            uVar6 = DAT_005f1d3c;
                        } else {
                            _DAT_005f1d64 = (float)uVar6;
                            _DAT_005f1d4c = _DAT_005f1d64 * DAT_0052a350[DAT_005f1d3c];
                            uVar6 = DAT_005f1d3c - uVar6;
                        }
                        _DAT_005f1d60 = (float)uVar6;
                        DAT_005f1d84 = (DAT_005f1d78 >> (DAT_005f262c & 0x1f)) + DAT_005f258c;
                        loc.iStack_10 = piVar7[3];
                        piVar5 = (int *)*piVar7;
                        uVar6 = (unsigned int)piVar7[1];
                        iVar8 = piVar7[2];
                        loc.fStack_14 = (float)(int)uVar6;
                        F30(0x34, int) = piVar7[4];
                        if (piVar5 == (int *)0) {
                            fVar1 = F30(0x28, float) * loc.fStack_14;
                        } else {
                            do {
                                iVar3 = piVar5[1];
                                if (iVar8 <= iVar3) break;
                                if ((int)uVar6 < iVar3) {
                                    FUN_00467570(uVar6, iVar3,
                                                 (float)(int)uVar6 * F30(0x28, float) + fStack_28);
                                    uVar6 = (unsigned int)piVar5[2];
                                    iVar8 = piVar7[2];
                                    if (iVar8 <= (int)uVar6) break;
                                } else {
                                    uVar4 = (unsigned int)piVar5[2];
                                    if (((int)uVar6 < (int)uVar4) && (uVar6 = uVar4, iVar8 <= (int)uVar4))
                                        goto LAB_00466338;
                                }
                                piVar5 = (int *)*piVar5;
                            } while (piVar5 != (int *)0);
                            if (iVar8 <= (int)uVar6) goto LAB_00466338;
                            fVar1 = (float)(int)uVar6 * F30(0x28, float);
                        }
                        FUN_00467570(uVar6, iVar8, fVar1 + fStack_28);
                    }
LAB_00466338:
                    fStack_28 = F30(0x2c, float) + fStack_28;
                    DAT_005f1d44 = DAT_005f1d44 + DAT_0070acc8;
                    DAT_005f1d78 = DAT_005f1d78 + 1;
                    iVar2 = iVar2 + 4;
                } while (DAT_005f1d78 < F30(0x44, unsigned int));
            }
            local_24 = local_24 + 0x70;
            local_20 = local_20 + 1;
        } while (local_20 < DAT_006a8850);
    }
    if (DAT_00732e8f == '\0') {
        FUN_004a6f90();
    }
    return;
}
