/* SPDX-License-Identifier: MIT */
/* z_xvt 004b5a2c */
void __stdcall FUN_0045bbe0(void);
void __stdcall FUN_004b67f9(void);

extern int DAT_0052f1d0;
extern void *DAT_0052f1f8;
extern void *DAT_00646e18;
extern void *DAT_0064ee44;
extern void *DAT_0064ee48;
extern void *DAT_0064eecc;
extern void *DAT_006572e0;
extern void *DAT_00742818;

void __stdcall FUN_004b5a2c(void)
{
    if (DAT_0052f1d0 == 0) {
        FUN_0045bbe0();
    }
    else {
        if (DAT_0052f1f8 != 0) {
            (*(void (**)(void *))((char *)*(void **)DAT_0052f1f8 + 8))(DAT_0052f1f8);
            DAT_0052f1f8 = 0;
        }
        FUN_004b67f9();
        if (DAT_00646e18 != 0) {
            (*(void (**)(void *))((char *)*(void **)DAT_00646e18 + 8))(DAT_00646e18);
            DAT_00646e18 = 0;
        }
        if (DAT_0064ee44 != 0) {
            (*(void (**)(void *))((char *)*(void **)DAT_0064ee44 + 8))(DAT_0064ee44);
            DAT_0064ee44 = 0;
        }
        if (DAT_0064ee48 != 0) {
            (*(void (**)(void *))((char *)*(void **)DAT_0064ee48 + 8))(DAT_0064ee48);
            DAT_0064ee48 = 0;
        }
        if (DAT_0064eecc != 0) {
            (*(void (**)(void *, int, void *))((char *)*(void **)DAT_006572e0 + 0x20))(DAT_006572e0, 0, DAT_0064eecc);
            (*(void (**)(void *))((char *)*(void **)DAT_0064eecc + 8))(DAT_0064eecc);
            DAT_0064eecc = 0;
        }
        if (DAT_00742818 != 0) {
            (*(void (**)(void *))((char *)*(void **)DAT_00742818 + 8))(DAT_00742818);
            DAT_00742818 = 0;
        }
        FUN_0045bbe0();
        DAT_0052f1d0 = 0;
    }
    return;
}
