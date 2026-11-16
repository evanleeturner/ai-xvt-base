/* SPDX-License-Identifier: MIT */
/* z_bop 004865e0 */
/* Unresolved evidence:
   - __ftol is called with its x87 argument hidden by the decompiler. The float
     expression that feeds it is not recoverable from this output, so the call
     is kept as shown and its input is unresolved.
   - The calling conventions and parameter types of the callees are not
     confirmed. They are declared from the decompiled call shapes.
   - Global types are inferred from use only. Pointer-sized globals are held as int. */

extern int DAT_00527e90;
extern char DAT_00a0813f;
extern int DAT_00999454;
extern int DAT_00999450;
extern unsigned char DAT_0099944a[4];   /* address 0x99944a is not 4-aligned: packed, read via memcpy */
extern int DAT_005233ac;
extern float DAT_005233d8;
extern int DAT_00527378[];
extern float DAT_005270b8[];
extern int DAT_00612280;
extern unsigned int DAT_00612284;
extern unsigned int DAT_0061228c;
extern float DAT_00612294;
extern float _DAT_0061229c;
extern float _DAT_006122b0;
extern float _DAT_006122b4;
extern unsigned int DAT_006122c8;
extern unsigned int DAT_006122d4;
extern unsigned int DAT_00612adc;
extern int _DAT_00612b58;
extern float _DAT_00612b5c;
extern float _DAT_00612b60;
extern int DAT_00612b64;
extern int DAT_00612b68;
extern int DAT_00612b6c;
extern int DAT_00612b70;
extern int DAT_00612b74;
extern unsigned char DAT_00612b7c;
extern int DAT_009a7b60;
extern int DAT_009a8e3c;
extern float DAT_009d8c08;

void FUN_004206a0(void);
void FUN_0040b2c0(void);
void FUN_004aba50(void);
long long __ftol(void);
void FUN_004879d0(int, int, float);
void FUN_004abe50(void);

void __stdcall FUN_004865e0(void)
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
    float local_28;
    int local_24;
    int local_20;
    unsigned char local_1c[8];
    float local_14;
    int local_10;
    int iBase;

    FUN_004206a0();
    if (DAT_00527e90 != 0) {
        FUN_0040b2c0();
        return;
    }
    if (DAT_00a0813f == '\0') {
        FUN_004aba50();
    }
    local_20 = DAT_00999454;
    _DAT_00612b58 = 0;
    if (DAT_00999454 < DAT_00999450) {
        local_24 = (int)((unsigned int)DAT_00999454 * 0x70u);
        do {
            __builtin_memcpy(&iBase, DAT_0099944a, 4);
            DAT_00612280 = (int)((unsigned int)iBase + (unsigned int)local_24);
            DAT_006122c8 = *(unsigned int *)(DAT_00612280 + 0x40);
            *(int *)(DAT_00612280 + 0x38) = (int)local_1c;
            iVar8 = 0;
            piVar7 = (int *)(DAT_00612280 + 4);
            iVar2 = *(int *)(*(int *)(DAT_00612280 + 4) + 0xa8);
            uVar6 = *(unsigned int *)(iVar2 + 0x14);
            uVar4 = *(unsigned int *)(iVar2 + 0x10);
            if ((DAT_005233ac != 0) &&
                (uVar6 * uVar4 - *(unsigned int *)(iVar2 + 8) == 0)) {
                lVar9 = __ftol();
                iVar2 = (int)lVar9;
                for (; ((0x100 < iVar2 && (uVar4 != 8)) && (uVar6 != 8));
                     uVar6 = (unsigned int)((int)uVar6 >> 1)) {
                    iVar2 = iVar2 >> 2;
                    iVar3 = (int)(uVar6 * uVar4);
                    uVar4 = (unsigned int)((int)uVar4 >> 1);
                    iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar3);
                }
            }
            _DAT_00612b5c = (float)(int)uVar4;
            _DAT_00612b60 = (float)(int)uVar6;
            DAT_00612b64 = DAT_00527378[(int)(uVar4 & 0xfffffff3u) >> 2];
            DAT_00612b74 = (int)(uVar6 * uVar4 - 1u);
            DAT_00612b68 = DAT_00527378[(int)(uVar6 & 0xfffffff3u) >> 2];
            DAT_00612b6c = *(int *)(*piVar7 + 0xb0);
            DAT_00612b70 = (int)((unsigned int)*(int *)(*piVar7 + 0xac) + (unsigned int)iVar8);
            _DAT_00612b58 = *piVar7;
            local_28 = (float)DAT_006122c8 * *(float *)(DAT_00612280 + 0x2c) +
                       *(float *)(DAT_00612280 + 0x30);
            DAT_00612294 = (float)(int)((unsigned int)DAT_009a7b60 + DAT_006122c8) * DAT_009d8c08 +
                           (float)DAT_009a8e3c * DAT_005233d8;
            if (DAT_006122c8 < *(unsigned int *)(DAT_00612280 + 0x44)) {
                iVar2 = 0;
                do {
                    piVar7 = *(int **)(*(int *)(DAT_00612280 + 0x68) + iVar2);
                    if (piVar7 != (int *)0) {
                        uVar6 = DAT_006122c8 & DAT_00612284;
                        if (uVar6 == 0) {
                            _DAT_0061229c = 0.0f;
                            _DAT_006122b4 = 0.0f;
                            uVar6 = DAT_0061228c;
                        } else {
                            _DAT_006122b4 = (float)uVar6;
                            _DAT_0061229c = _DAT_006122b4 * DAT_005270b8[DAT_0061228c];
                            uVar6 = DAT_0061228c - uVar6;
                        }
                        _DAT_006122b0 = (float)uVar6;
                        DAT_006122d4 = (DAT_006122c8 >> (DAT_00612b7c & 0x1f)) + DAT_00612adc;
                        local_10 = piVar7[3];
                        (void)local_10;
                        piVar5 = (int *)*piVar7;
                        uVar6 = (unsigned int)piVar7[1];
                        iVar8 = piVar7[2];
                        local_14 = (float)(int)uVar6;
                        *(int *)(DAT_00612280 + 0x34) = piVar7[4];
                        if (piVar5 == (int *)0) {
                            fVar1 = *(float *)(DAT_00612280 + 0x28) * local_14;
                        } else {
                            do {
                                iVar3 = piVar5[1];
                                if (iVar8 <= iVar3) break;
                                if ((int)uVar6 < iVar3) {
                                    FUN_004879d0((int)uVar6, iVar3,
                                                 (float)(int)uVar6 * *(float *)(DAT_00612280 + 0x28) + local_28);
                                    uVar6 = (unsigned int)piVar5[2];
                                    iVar8 = piVar7[2];
                                    if (iVar8 <= (int)uVar6) break;
                                } else {
                                    uVar4 = (unsigned int)piVar5[2];
                                    if (((int)uVar6 < (int)uVar4) && (uVar6 = uVar4, iVar8 <= (int)uVar4))
                                        goto LAB_004868ff;
                                }
                                piVar5 = (int *)*piVar5;
                            } while (piVar5 != (int *)0);
                            if (iVar8 <= (int)uVar6) goto LAB_004868ff;
                            fVar1 = (float)(int)uVar6 * *(float *)(DAT_00612280 + 0x28);
                        }
                        FUN_004879d0((int)uVar6, iVar8, fVar1 + local_28);
                    }
LAB_004868ff:
                    local_28 = *(float *)(DAT_00612280 + 0x2c) + local_28;
                    DAT_00612294 = DAT_00612294 + DAT_009d8c08;
                    DAT_006122c8 = DAT_006122c8 + 1;
                    iVar2 = iVar2 + 4;
                } while (DAT_006122c8 < *(unsigned int *)(DAT_00612280 + 0x44));
            }
            local_24 = (int)((unsigned int)local_24 + 0x70u);
            local_20 = local_20 + 1;
        } while (local_20 < DAT_00999450);
    }
    if (DAT_00a0813f == '\0') {
        FUN_004abe50();
    }
    return;
}
