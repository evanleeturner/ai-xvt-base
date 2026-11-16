/* SPDX-License-Identifier: MIT */
/* z_bop 004b7320 */
typedef struct _iobuf FILE;

void *__cdecl _malloc(size_t);
void __cdecl _free(void *);
int __cdecl FUN_004cc6c0(FILE *param_1, void *param_2, size_t param_3);

unsigned int __cdecl FUN_004b7320(FILE *param_1, int param_2, unsigned int param_3, int param_4)
{
    unsigned char *pbVar1;
    unsigned char bVar2;
    unsigned char bVar3;
    unsigned short uVar4;
    _Bool bVar5;
    int iVar6;
    unsigned int uVar7;
    int iVar8;
    void *_Memory;
    short sVar9;
    int iVar10;
    size_t local_c;
    int local_8;
    unsigned char local_4[4];

    uVar7 = (unsigned int)(*(int *)((unsigned int)param_4 + 4u) >> 0x1f);
    local_c = (((((unsigned int)*(int *)((unsigned int)param_4 + 4u) ^ uVar7) - uVar7) & 3u) ^ uVar7) - uVar7;
    if (local_c != 0) {
        local_c = 4u - local_c;
    }
    _Memory = (void *)0;
    if (*(int *)((unsigned int)param_4 + 0x10u) != 0) {
        _Memory = _malloc(*(size_t *)((unsigned int)param_4 + 0x14u));
        if (_Memory == (void *)0) {
            return 0;
        }
    }
    if (*(int *)((unsigned int)param_4 + 0x10u) == 0) {
        sVar9 = 0;
        if (0 < *(int *)((unsigned int)param_4 + 8u)) {
            do {
                iVar8 = (int)sVar9;
                sVar9 = (short)(sVar9 + 1);
                FUN_004cc6c0(param_1,
                             (void *)((((unsigned int)*(int *)((unsigned int)param_4 + 8u) - (unsigned int)iVar8) - 1u) *
                                      *(size_t *)((unsigned int)param_4 + 4u) + (unsigned int)param_2),
                             *(size_t *)((unsigned int)param_4 + 4u));
                FUN_004cc6c0(param_1, local_4, local_c);
            } while ((int)sVar9 < *(int *)((unsigned int)param_4 + 8u));
        }
    }
    else if (*(int *)((unsigned int)param_4 + 0x10u) == 1) {
        FUN_004cc6c0(param_1, _Memory, *(size_t *)((unsigned int)param_4 + 0x14u));
        bVar5 = 0;
        iVar10 = (int)(((unsigned int)*(int *)((unsigned int)param_4 + 8u) - 1u) *
                       (unsigned int)*(int *)((unsigned int)param_4 + 4u));
        iVar8 = 0;
        local_8 = iVar10;
        do {
            bVar2 = *(unsigned char *)((unsigned int)_Memory + (unsigned int)iVar8);
            bVar3 = *(unsigned char *)((unsigned int)_Memory + (unsigned int)iVar8 + 1u);
            if (bVar2 == 0) {
                iVar6 = (int)((unsigned int)iVar8 + 2u);
                uVar4 = (unsigned short)bVar3;
                if (bVar3 == 0) {
                    iVar10 = (int)((unsigned int)local_8 - (unsigned int)*(int *)((unsigned int)param_4 + 4u));
                    local_8 = iVar10;
                }
                else if (uVar4 == 1) {
                    bVar5 = 1;
                }
                else if (uVar4 == 2) {
                    pbVar1 = (unsigned char *)((unsigned int)_Memory + (unsigned int)iVar6);
                    iVar6 = (int)((unsigned int)iVar8 + 4u);
                    iVar8 = (int)((unsigned int)local_8 -
                                  (unsigned int)*(unsigned char *)((unsigned int)_Memory + (unsigned int)iVar8 + 3u) *
                                  (unsigned int)*(int *)((unsigned int)param_4 + 4u));
                    iVar10 = (int)(((unsigned int)iVar10 - (unsigned int)local_8) + (unsigned int)*pbVar1 +
                                   (unsigned int)iVar8);
                    local_8 = iVar8;
                }
                else {
                    sVar9 = 0;
                    if (bVar3 != 0) {
                        do {
                            sVar9 = (short)(sVar9 + 1);
                            *(unsigned char *)((unsigned int)param_2 + (unsigned int)iVar10) =
                                *(unsigned char *)((unsigned int)_Memory + (unsigned int)iVar6);
                            iVar6 = (int)((unsigned int)iVar6 + 1u);
                            iVar10 = (int)((unsigned int)iVar10 + 1u);
                        } while (sVar9 < (short)uVar4);
                    }
                    if ((bVar3 & 1) != 0) {
                        iVar6 = (int)((unsigned int)iVar6 + 1u);
                    }
                }
            }
            else {
                sVar9 = 0;
                iVar6 = (int)((unsigned int)iVar8 + 2u);
                if (bVar2 != 0) {
                    do {
                        sVar9 = (short)(sVar9 + 1);
                        *(unsigned char *)((unsigned int)param_2 + (unsigned int)iVar10) = bVar3;
                        iVar10 = (int)((unsigned int)iVar10 + 1u);
                    } while (sVar9 < (short)(unsigned short)bVar2);
                }
            }
            iVar8 = iVar6;
        } while (!bVar5);
    }
    if (_Memory != (void *)0) {
        _free(_Memory);
    }
    return 1;
}
