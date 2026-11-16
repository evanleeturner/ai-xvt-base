/* SPDX-License-Identifier: MIT */
/* z_bop 004244e0 */
extern int DAT_00527e94;
extern int DAT_00527e90;
extern int *DAT_009a1fe8;
extern int DAT_0051c4a8;
extern int DAT_009ecc34;
extern int DAT_0051c4ac;
extern char DAT_009e967a;
extern char DAT_009e9685;
extern char DAT_009e9a5b;
extern char DAT_009e9a5f;
extern char DAT_009e9a63;
extern int DAT_00552c80[0x400];
extern int DAT_00553c80[0x400];
extern int DAT_00551c80[0x400];
extern int DAT_00550c80[0x400];
extern int DAT_00554c80[0x400];
extern float _DAT_0051c340;
extern float _DAT_0051c34c;
extern float _DAT_0051c350;
extern float _DAT_0051c358;
extern float _DAT_0051c35c;
extern float _DAT_0051c364;

void FUN_0042e4d0(unsigned int a, unsigned int b, int c);
int _rand(void);
unsigned int FUN_0046abf0(unsigned int a, unsigned int b);
long long __ftol(void);
void FUN_00424410(void);

void __stdcall FUN_004244e0(void)
{
    int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    int iVar4;
    unsigned int uVar9;
    int local_e8;
    int local_e4;
    unsigned int local_d8[8];
    unsigned short local_d8s;
    unsigned char local_d8c;
    unsigned int local_b4[45];
    unsigned char local_b4c;
    unsigned int *puVar5;
    unsigned int *puVar6;
    long long lVar8;
    unsigned short tmp16;
    unsigned int tmp32;
    unsigned int *tmpp;

    uVar1 = DAT_00527e94;
    local_e4 = 0x400;
    if (DAT_00527e90 == 0) {
        local_e4 = 0x200;
    }
    DAT_00527e94 = 0;

    puVar5 = (unsigned int *)DAT_009a1fe8;
    puVar6 = local_d8;
    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
    }
    local_d8s = *(unsigned short *)puVar5;
    local_d8c = *((unsigned char *)puVar5 + 2);

    __builtin_memcpy(&tmpp, (unsigned char *)DAT_009a1fe8 + 0x1f, 4);
    puVar5 = tmpp;
    puVar6 = local_b4;
    for (iVar4 = 0x2c; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
    }
    local_b4c = *(unsigned char *)puVar5;

    if (DAT_0051c4a8 != 0) {
        __builtin_memcpy(&tmp16, (unsigned char *)&DAT_009e967a + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 2);
        if ((short)tmp16 == 1) {
            uVar9 = 0x50;
        } else {
            uVar9 = 0x53;
        }
        FUN_0042e4d0(uVar9, 0xffffffff, DAT_009ecc34);
        local_e8 = 0;
        while (local_e8 < 0x1000) {
            do {
                uVar9 = (unsigned int)_rand();
                uVar2 = (unsigned int)_rand();
                uVar3 = (unsigned int)_rand();
                iVar4 = (int)((uVar3 & 3) + 2);
                uVar9 = (unsigned int)((int)((unsigned int)iVar4 * (uVar9 & 0x3fff)) >> 3);
                uVar2 = (unsigned int)((int)((unsigned int)iVar4 * (uVar2 & 0x3fff)) >> 3);
            } while ((unsigned int)((int)(uVar9 + uVar2) >> 1) < 8);
            *(int *)((char *)DAT_00552c80 + local_e8) = (int)((unsigned int)((int)(uVar9 + uVar2) >> 8) + 1u);
            uVar3 = (unsigned int)_rand();
            if ((uVar3 & 0x1000) == 0) {
                uVar9 = 0u - uVar9;
            }
            *(unsigned int *)((char *)DAT_00553c80 + local_e8) = uVar9;
            uVar9 = (unsigned int)_rand();
            if ((uVar9 & 0x1000) == 0) {
                uVar2 = 0u - uVar2;
            }
            *(unsigned int *)((char *)DAT_00551c80 + local_e8) = uVar2;
            *(int *)((char *)DAT_00550c80 + local_e8) = 0x4000;
            uVar9 = FUN_0046abf0(*(unsigned int *)((char *)DAT_00551c80 + local_e8),
                                 *(unsigned int *)((char *)DAT_00553c80 + local_e8));
            *(unsigned int *)((char *)DAT_00554c80 + local_e8) = (uVar9 & 0xffff) + 0x4000;
            local_e8 = local_e8 + 4;
        }
        DAT_0051c4a8 = 0;
    }

    iVar4 = 0;
    if (local_e4 != 0) {
        local_e8 = local_e4;
        do {
            __builtin_memcpy(&tmp32, (unsigned char *)&DAT_009e9a5b + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
            DAT_009a1fe8[1] = (int)(tmp32 + (unsigned int)*(int *)((char *)DAT_00553c80 + iVar4));
            __builtin_memcpy(&tmp32, (unsigned char *)&DAT_009e9a5f + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
            DAT_009a1fe8[2] = (int)(tmp32 + (unsigned int)*(int *)((char *)DAT_00550c80 + iVar4));
            __builtin_memcpy(&tmp32, (unsigned char *)&DAT_009e9a63 + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
            DAT_009a1fe8[3] = (int)(tmp32 + (unsigned int)*(int *)((char *)DAT_00551c80 + iVar4));
            *((char *)DAT_009a1fe8 + 3) = (char)0x89;
            *((char *)DAT_009a1fe8 + 2) = 7;
            *(short *)(DAT_009a1fe8 + 5) = (short)*(int *)((char *)DAT_00554c80 + iVar4);
            *(short *)(DAT_009a1fe8 + 4) = 0;
            *(short *)((char *)DAT_009a1fe8 + 0x12) = 0x4000;
            _DAT_0051c340 = (float)*(int *)((char *)DAT_00552c80 + iVar4);
            _DAT_0051c358 = (float)(int)(0u - (unsigned int)*(int *)((char *)DAT_00552c80 + iVar4));
            __builtin_memcpy(&uVar9, (unsigned char *)&DAT_009e9685 + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
            _DAT_0051c34c = _DAT_0051c340;
            _DAT_0051c364 = _DAT_0051c358;
            if (uVar9 < 0x1d8) {
                long double fVar7 = (long double)(uVar9 >> 2) * (long double)(uVar9 >> 2);
                _DAT_0051c350 = (float)fVar7;
                _DAT_0051c35c = (float)fVar7;
                DAT_009a1fe8[2] = (int)((unsigned int)DAT_009a1fe8[2] + uVar9 * 0xfffffff0u);
                DAT_0051c4ac = 1;
            } else {
                if (DAT_0051c4ac != 0) {
                    __builtin_memcpy(&tmp16, (unsigned char *)&DAT_009e967a + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 2);
                    if ((short)tmp16 == 1) {
                        uVar9 = 0x51;
                    } else {
                        uVar9 = 0x54;
                    }
                    FUN_0042e4d0(uVar9, 0xffffffff, DAT_009ecc34);
                    DAT_0051c4ac = 0;
                }
                _DAT_0051c350 = 16000.0f;
                _DAT_0051c35c = 16000.0f;
                DAT_009a1fe8[2] = (int)((unsigned int)DAT_009a1fe8[2] + 0xffffe280u);
                {
                    int *pi = DAT_009a1fe8;
                    lVar8 = __ftol();
                    pi[2] = (int)((unsigned int)pi[2] - (unsigned int)(int)lVar8);
                }
            }
            iVar4 = iVar4 + 4;
            FUN_00424410();
            local_e8 = local_e8 + -1;
        } while (local_e8 != 0);
    }

    puVar5 = local_d8;
    puVar6 = (unsigned int *)DAT_009a1fe8;
    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
    }
    *(unsigned short *)puVar6 = local_d8s;
    *((unsigned char *)puVar6 + 2) = local_d8c;

    puVar5 = local_b4;
    __builtin_memcpy(&tmpp, (unsigned char *)DAT_009a1fe8 + 0x1f, 4);
    puVar6 = tmpp;
    for (iVar4 = 0x2c; iVar4 != 0; iVar4 = iVar4 + -1) {
        *puVar6 = *puVar5;
        puVar5 = puVar5 + 1;
        puVar6 = puVar6 + 1;
    }
    *(unsigned char *)puVar6 = local_b4c;
    DAT_00527e94 = uVar1;
    return;
}
