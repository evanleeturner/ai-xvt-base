/* SPDX-License-Identifier: MIT */
/* z_bop 00448da0 */
extern void FUN_004a9640(int);
extern void *_malloc(unsigned int);
extern void _free(void *);
extern void FUN_004abee0(void);
extern void FUN_004aba50(void);
extern void FUN_004abe50(void);
extern void (*DAT_009d8b6c)(void *, unsigned int, unsigned int, unsigned int, unsigned int);
extern unsigned int DAT_005233e0;
extern unsigned int DAT_005233dc;
extern unsigned char DAT_009a20ae;
extern unsigned int DAT_005233e4;
extern unsigned int DAT_005233e8;
extern void *DAT_005236a0;
extern unsigned int DAT_005233d8;
extern unsigned int DAT_005236a4;
extern unsigned int DAT_00527ed0;

void __stdcall FUN_00448da0(void)
{
    unsigned int uVar1;
    unsigned int uVar2;
    unsigned int uVar3;
    unsigned int uVar4;
    unsigned int uVar5;
    unsigned int uVar6;
    unsigned int iVar7;
    
    FUN_004a9640(0);
    uVar5 = (unsigned int)(DAT_005233e0 - DAT_005233dc) >> 1;
    uVar2 = (unsigned int)DAT_009a20ae;
    uVar1 = DAT_005233e0 + DAT_005233dc;
    uVar4 = DAT_005233e4 + DAT_005233e8;
    uVar6 = uVar5 + 2;
    iVar7 = uVar2 * 5 + 2;
    
    if (DAT_005236a0 == (void *)0) {
        DAT_005236a0 = _malloc(DAT_005233d8 * iVar7 * uVar6);
    }
    else {
        uVar3 = DAT_005233d8 * iVar7 * uVar6;
        if (uVar3 <= DAT_005236a4) {
            /* LAB_00448e71 */
        }
        else {
            _free(DAT_005236a0);
            DAT_005236a0 = _malloc(DAT_005233d8 * iVar7 * uVar6);
        }
    }
    
    if (DAT_005236a0 == (void *)0) {
        return;
    }
    DAT_005236a4 = DAT_005233d8 * iVar7 * uVar6;
    
    FUN_004abee0();
    DAT_00527ed0 = 0;
    FUN_004aba50();
    (*DAT_009d8b6c)(DAT_005236a0,
                   ((uVar1 >> 1) - uVar5 / 2) - 1 & 0xffff,
                   ((uVar4 >> 1) - (uVar2 * 5) / 2) - 1 & 0xffff,
                   uVar6 & 0xffff,
                   iVar7);
    FUN_004abe50();
    DAT_00527ed0 = 1;
    FUN_004abee0();
    return;
}
