/* SPDX-License-Identifier: MIT */
/* z_xvt 004f7a80 */
void __stdcall RaiseException(DWORD, DWORD, DWORD, const DWORD *);
void __cdecl FUN_004f8160(int param_1, int param_2, int param_3, unsigned short param_4);
void __cdecl FUN_004f82f0(int param_1, int param_2, int param_3, unsigned short param_4);

extern int DAT_00758830;
extern int DAT_0075964b;  /* unaligned (0x75964b) */
extern int DAT_007596b7;  /* unaligned, followed contiguously by bb, bf, c3 */

void __cdecl FUN_004f7a80(int param_1, int param_2, int param_3, int param_4, unsigned short param_5)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned char *puVar3;
    int iVar4;
    int iVar5;
    unsigned int uVar6;
    unsigned char *puVar7;
    int local_9;
    unsigned int local_4;
    int num_;
    int den_;
    int clip[4];
    int clipL, clipR, clipT, clipB, stride;
    unsigned char bv;

    iVar5 = 0x800;
    if (param_4 == param_2) {
        FUN_004f8160(param_1, param_3, param_2, param_5);
        return;
    }
    if (param_3 == param_1) {
        FUN_004f82f0(param_2, param_4, param_1, param_5);
        return;
    }
    __builtin_memcpy(clip, &DAT_007596b7, 16);
    __builtin_memcpy(&stride, &DAT_0075964b, 4);
    clipL = clip[0];
    clipR = clip[1];
    clipT = clip[2];
    clipB = clip[3];
    bv = (unsigned char)param_5;

    if (param_2 < param_4) {
        if (param_1 < param_3) {
            num_ = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
            den_ = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            local_4 = (unsigned int)(num_ / den_);
            local_9 = param_1 < clipL;
            if (local_9) {
                num_ = (int)(((unsigned)clipL - (unsigned)param_1) * 0x1000u);
                den_ = (int)local_4;
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                param_2 = (int)((unsigned)param_2 + (unsigned)(num_ / den_));
                param_1 = clipL;
            }
            iVar4 = param_2;
            if (param_2 < clipT) {
                param_1 = (int)((unsigned)param_1 + (unsigned)((int)(((unsigned)clipT - (unsigned)param_2) * local_4) >> 0xc));
                iVar4 = clipT;
            }
            local_9 = param_2 < clipT || local_9;
            if (clipR < param_3) {
                local_9 = 1;
                num_ = (int)(((unsigned)clipR - (unsigned)param_3) * 0x1000u);
                den_ = (int)local_4;
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                param_4 = (int)((unsigned)param_4 + (unsigned)(num_ / den_));
                param_3 = clipR;
            }
            if (clipB < param_4) {
                local_9 = 1;
                param_3 = (int)((unsigned)param_3 - (unsigned)((int)(((unsigned)param_4 - (unsigned)clipB) * local_4) >> 0xc));
                param_4 = clipB;
            }
            if (local_9) {
                if (param_3 < param_1) {
                    return;
                }
                if (param_4 < iVar4) {
                    return;
                }
                num_ = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
                den_ = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                local_4 = (unsigned int)(num_ / den_);
            }
            puVar3 = (unsigned char *)(unsigned int)((unsigned)iVar4 * (unsigned)stride + (unsigned)param_1 + (unsigned)DAT_00758830);
            uVar1 = (unsigned int)((int)local_4 >> 0xc);
            if (uVar1 == 0) {
                iVar4 = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
                if (iVar4 < 1) {
                    return;
                }
                do {
                    *puVar3 = bv;
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    if (0xfff < iVar5) {
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 + (unsigned)stride);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
            iVar4 = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
            if (iVar4 < 1) {
                return;
            }
            do {
                iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                uVar6 = uVar1;
                if (0xfff < iVar5) {
                    uVar6 = uVar1 + 1;
                    iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                }
                puVar7 = puVar3;
                for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                    puVar7[0] = bv;
                    puVar7[1] = bv;
                    puVar7[2] = bv;
                    puVar7[3] = bv;
                    puVar7 = puVar7 + 4;
                }
                for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                    *puVar7 = bv;
                    puVar7 = puVar7 + 1;
                }
                puVar3 = (unsigned char *)((unsigned int)puVar3 + (unsigned)stride + uVar6);
                iVar4 = iVar4 - 1;
            } while (iVar4 != 0);
            return;
        }
        if ((param_2 < param_4) && (param_3 < param_1)) {
            num_ = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
            den_ = (int)(((unsigned)param_4 - (unsigned)param_2) + 1u);
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            local_4 = (unsigned int)(num_ / den_);
            local_9 = clipR < param_1;
            if (local_9) {
                num_ = (int)(((unsigned)param_1 - (unsigned)clipR) * 0x1000u);
                den_ = (int)local_4;
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                param_2 = (int)((unsigned)param_2 + (unsigned)(num_ / den_));
                param_1 = clipR;
            }
            iVar4 = param_2;
            if (param_2 < clipT) {
                param_1 = (int)((unsigned)param_1 - (unsigned)((int)(((unsigned)clipT - (unsigned)param_2) * local_4) >> 0xc));
                iVar4 = clipT;
            }
            local_9 = param_2 < clipT || local_9;
            if (param_3 < clipL) {
                local_9 = 1;
                num_ = (int)(((unsigned)param_3 - (unsigned)clipL) * 0x1000u);
                den_ = (int)local_4;
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                param_4 = (int)((unsigned)param_4 + (unsigned)(num_ / den_));
                param_3 = clipL;
            }
            if (clipB < param_4) {
                local_9 = 1;
                param_3 = (int)((unsigned)param_3 + (unsigned)((int)(((unsigned)param_4 - (unsigned)clipB) * local_4) >> 0xc));
                param_4 = clipB;
            }
            if (local_9) {
                if (param_1 < param_3) {
                    return;
                }
                if (param_4 < iVar4) {
                    return;
                }
                num_ = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
                den_ = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
                if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
                local_4 = (unsigned int)(num_ / den_);
            }
            puVar3 = (unsigned char *)(unsigned int)((unsigned)param_4 * (unsigned)stride + (unsigned)param_3 + (unsigned)DAT_00758830);
            uVar1 = (unsigned int)((int)local_4 >> 0xc);
            if (uVar1 == 0) {
                iVar4 = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
                if (iVar4 < 1) {
                    return;
                }
                do {
                    *puVar3 = bv;
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    if (0xfff < iVar5) {
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 - (unsigned)stride);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
            iVar4 = (int)(((unsigned)param_4 - (unsigned)iVar4) + 1u);
            if (iVar4 < 1) {
                return;
            }
            do {
                iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                uVar6 = uVar1;
                if (0xfff < iVar5) {
                    uVar6 = uVar1 + 1;
                    iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                }
                puVar7 = puVar3;
                for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                    puVar7[0] = bv;
                    puVar7[1] = bv;
                    puVar7[2] = bv;
                    puVar7[3] = bv;
                    puVar7 = puVar7 + 4;
                }
                for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                    *puVar7 = bv;
                    puVar7 = puVar7 + 1;
                }
                puVar3 = (unsigned char *)((unsigned int)puVar3 + (uVar6 - (unsigned)stride));
                iVar4 = iVar4 - 1;
            } while (iVar4 != 0);
            return;
        }
    }
    if ((param_4 < param_2) && (param_1 < param_3)) {
        num_ = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
        den_ = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
        if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
        local_4 = (unsigned int)(num_ / den_);
        local_9 = param_1 < clipL;
        if (local_9) {
            num_ = (int)(((unsigned)param_1 - (unsigned)clipL) * 0x1000u);
            den_ = (int)local_4;
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(num_ / den_));
            param_1 = clipL;
        }
        iVar4 = param_2;
        if (clipB < param_2) {
            param_1 = (int)((unsigned)param_1 + (unsigned)((int)(((unsigned)param_2 - (unsigned)clipB) * local_4) >> 0xc));
            iVar4 = clipB;
        }
        local_9 = clipB < param_2 || local_9;
        if (clipR < param_3) {
            local_9 = 1;
            num_ = (int)(((unsigned)param_3 - (unsigned)clipR) * 0x1000u);
            den_ = (int)local_4;
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(num_ / den_));
            param_3 = clipR;
        }
        if (param_4 < clipT) {
            local_9 = 1;
            param_3 = (int)((unsigned)param_3 - (unsigned)((int)(((unsigned)clipT - (unsigned)param_4) * local_4) >> 0xc));
            param_4 = clipT;
        }
        if (local_9) {
            if (param_3 < param_1) {
                return;
            }
            if (iVar4 < param_4) {
                return;
            }
            num_ = (int)((((unsigned)param_3 - (unsigned)param_1) + 1u) * 0x1000u);
            den_ = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            local_4 = (unsigned int)(num_ / den_);
        }
        puVar3 = (unsigned char *)(unsigned int)((unsigned)iVar4 * (unsigned)stride + (unsigned)param_1 + (unsigned)DAT_00758830);
        uVar1 = (unsigned int)((int)local_4 >> 0xc);
        if (uVar1 == 0) {
            iVar4 = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (0 < iVar4) {
                do {
                    *puVar3 = bv;
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    if (0xfff < iVar5) {
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 - (unsigned)stride);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        } else {
            iVar4 = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    uVar6 = uVar1;
                    if (0xfff < iVar5) {
                        uVar6 = uVar1 + 1;
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                    }
                    puVar7 = puVar3;
                    for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                        puVar7[0] = bv;
                        puVar7[1] = bv;
                        puVar7[2] = bv;
                        puVar7[3] = bv;
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar7 = bv;
                        puVar7 = puVar7 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 + (uVar6 - (unsigned)stride));
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        }
    } else {
        num_ = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
        den_ = (int)(((unsigned)param_2 - (unsigned)param_4) + 1u);
        if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
        local_4 = (unsigned int)(num_ / den_);
        local_9 = clipR < param_1;
        if (local_9) {
            num_ = (int)(((unsigned)clipR - (unsigned)param_1) * 0x1000u);
            den_ = (int)local_4;
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            param_2 = (int)((unsigned)param_2 + (unsigned)(num_ / den_));
            param_1 = clipR;
        }
        iVar4 = param_2;
        if (clipB < param_2) {
            param_1 = (int)((unsigned)param_1 - (unsigned)((int)(((unsigned)param_2 - (unsigned)clipB) * local_4) >> 0xc));
            iVar4 = clipB;
        }
        local_9 = clipB < param_2 || local_9;
        if (param_3 < clipL) {
            local_9 = 1;
            num_ = (int)(((unsigned)clipL - (unsigned)param_3) * 0x1000u);
            den_ = (int)local_4;
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            param_4 = (int)((unsigned)param_4 + (unsigned)(num_ / den_));
            param_3 = clipL;
        }
        if (param_4 < clipT) {
            local_9 = 1;
            param_3 = (int)((unsigned)param_3 + (unsigned)((int)(((unsigned)clipT - (unsigned)param_4) * local_4) >> 0xc));
            param_4 = clipT;
        }
        if (local_9) {
            if (param_1 < param_3) {
                return;
            }
            if (iVar4 < param_4) {
                return;
            }
            num_ = (int)((((unsigned)param_1 - (unsigned)param_3) + 1u) * 0x1000u);
            den_ = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (den_ == 0 || (num_ == (int)0x80000000 && den_ == -1)) goto fault;
            local_4 = (unsigned int)(num_ / den_);
        }
        puVar3 = (unsigned char *)(unsigned int)((unsigned)param_4 * (unsigned)stride + (unsigned)param_3 + (unsigned)DAT_00758830);
        uVar1 = (unsigned int)((int)local_4 >> 0xc);
        if (uVar1 == 0) {
            iVar4 = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (0 < iVar4) {
                do {
                    *puVar3 = bv;
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    if (0xfff < iVar5) {
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                        puVar3 = puVar3 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 + (unsigned)stride);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
            }
        } else {
            iVar4 = (int)(((unsigned)iVar4 - (unsigned)param_4) + 1u);
            if (0 < iVar4) {
                do {
                    iVar5 = (int)((unsigned)iVar5 + (local_4 & 0xfff));
                    uVar6 = uVar1;
                    if (0xfff < iVar5) {
                        uVar6 = uVar1 + 1;
                        iVar5 = (int)((unsigned)iVar5 - 0x1000u);
                    }
                    puVar7 = puVar3;
                    for (uVar2 = uVar6 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
                        puVar7[0] = bv;
                        puVar7[1] = bv;
                        puVar7[2] = bv;
                        puVar7[3] = bv;
                        puVar7 = puVar7 + 4;
                    }
                    for (uVar2 = uVar6 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
                        *puVar7 = bv;
                        puVar7 = puVar7 + 1;
                    }
                    puVar3 = (unsigned char *)((unsigned int)puVar3 + (unsigned)stride + uVar6);
                    iVar4 = iVar4 - 1;
                } while (iVar4 != 0);
                return;
            }
        }
    }
    return;

fault:
    RaiseException(0xC0000094UL, 0, 0, NULL);
    return;
}
