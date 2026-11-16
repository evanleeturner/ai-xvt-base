/* SPDX-License-Identifier: MIT */
/* z_bop 00427e90 */
extern int DAT_00a0049c, DAT_00a004a8, DAT_00a0048c, DAT_00a004cc, DAT_00a00494, DAT_00a00488;
extern int DAT_00a004a4, DAT_00a004b8, DAT_00a00480;
extern int DAT_00a004b0, DAT_009fe7f4, DAT_00a00490;
extern int DAT_00a07c68, DAT_00a07c6c, DAT_00a07cc4;
extern int DAT_00a004d0, DAT_00a004ac, DAT_00a004b4;
extern int DAT_009d77ac, DAT_009d77b8, DAT_009d77b4, DAT_009d77b0;
extern int DAT_009d7690, DAT_009d768c, DAT_009d7688, DAT_009d77a4, DAT_009d77a0;
extern int DAT_009d12ec, DAT_009d12f0, DAT_009d1304;
extern int DAT_00a081e0, DAT_00a081e4, DAT_00a08150;
extern char DAT_009ec476;

#define _DAT_009d7688 DAT_009d7688
#define _DAT_009d77a4 DAT_009d77a4
#define _DAT_009d77a0 DAT_009d77a0
#define _DAT_009d12ec DAT_009d12ec
#define _DAT_009d12f0 DAT_009d12f0
#define _DAT_009d1304 DAT_009d1304
#define _DAT_00a081e0 DAT_00a081e0
#define _DAT_00a081e4 DAT_00a081e4
#define _DAT_00a08150 DAT_00a08150

void __stdcall FUN_00427e90(void)
{
    int iVar1;

    iVar1 = (int)((unsigned int)DAT_00a0049c * (unsigned int)DAT_00a004a8 +
                  (unsigned int)DAT_00a0048c * (unsigned int)DAT_00a004cc +
                  (unsigned int)DAT_00a00494 * (unsigned int)DAT_00a00488);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d77ac = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a0049c * (unsigned int)DAT_00a004a4 +
                  (unsigned int)DAT_00a0048c * (unsigned int)DAT_00a004b8 +
                  (unsigned int)DAT_00a00494 * (unsigned int)DAT_00a00480);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d77b8 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a0049c * (unsigned int)DAT_00a004b0 +
                  (unsigned int)DAT_00a0048c * (unsigned int)DAT_009fe7f4 +
                  (unsigned int)DAT_00a00494 * (unsigned int)DAT_00a00490);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d77b4 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)DAT_00a004a8 +
                  (unsigned int)DAT_00a07c6c * (unsigned int)DAT_00a004cc +
                  (unsigned int)DAT_00a07cc4 * (unsigned int)DAT_00a00488);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d77b0 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)DAT_00a004a4 +
                  (unsigned int)DAT_00a07c6c * (unsigned int)DAT_00a004b8 +
                  (unsigned int)DAT_00a07cc4 * (unsigned int)DAT_00a00480);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d7690 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)DAT_00a004b0 +
                  (unsigned int)DAT_00a07c6c * (unsigned int)DAT_009fe7f4 +
                  (unsigned int)DAT_00a07cc4 * (unsigned int)DAT_00a00490);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    DAT_009d768c = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)DAT_00a004a8 +
                  (unsigned int)DAT_00a004ac * (unsigned int)DAT_00a004cc +
                  (unsigned int)DAT_00a004b4 * (unsigned int)DAT_00a00488);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    _DAT_009d7688 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)DAT_00a004a4 +
                  (unsigned int)DAT_00a004ac * (unsigned int)DAT_00a004b8 +
                  (unsigned int)DAT_00a004b4 * (unsigned int)DAT_00a00480);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    _DAT_009d77a4 = iVar1 >> 0xf;

    iVar1 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)DAT_00a004b0 +
                  (unsigned int)DAT_00a004ac * (unsigned int)DAT_009fe7f4 +
                  (unsigned int)DAT_00a004b4 * (unsigned int)DAT_00a00490);
    if (0x3fffffff < iVar1) {
        iVar1 = 0x3fffffff;
    }
    if (iVar1 < -0x3fffffff) {
        iVar1 = -0x3fff0000;
    }
    _DAT_009d77a0 = iVar1 >> 0xf;

    if (DAT_009ec476 != '\0') {
        iVar1 = (int)((unsigned int)DAT_00a0049c * (unsigned int)_DAT_009d12ec +
                      (unsigned int)DAT_00a0048c * (unsigned int)_DAT_009d12f0 +
                      (unsigned int)DAT_00a00494 * (unsigned int)_DAT_009d1304);
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        _DAT_00a081e0 = iVar1 >> 0xf;

        iVar1 = (int)((unsigned int)DAT_00a07c68 * (unsigned int)_DAT_009d12ec +
                      (unsigned int)DAT_00a07c6c * (unsigned int)_DAT_009d12f0 +
                      (unsigned int)DAT_00a07cc4 * (unsigned int)_DAT_009d1304);
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        _DAT_00a081e4 = iVar1 >> 0xf;

        iVar1 = (int)((unsigned int)DAT_00a004d0 * (unsigned int)_DAT_009d12ec +
                      (unsigned int)DAT_00a004ac * (unsigned int)_DAT_009d12f0 +
                      (unsigned int)DAT_00a004b4 * (unsigned int)_DAT_009d1304);
        if (0x3fffffff < iVar1) {
            iVar1 = 0x3fffffff;
        }
        if (iVar1 < -0x3fffffff) {
            iVar1 = -0x3fff0000;
        }
        _DAT_00a08150 = iVar1 >> 0xf;
        return;
    }
    _DAT_00a081e0 = _DAT_009d12ec;
    _DAT_00a081e4 = _DAT_009d12f0;
    _DAT_00a08150 = _DAT_009d1304;
    return;
}
