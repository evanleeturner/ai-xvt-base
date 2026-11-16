/* SPDX-License-Identifier: MIT */
/* z_bop 004d3a00 */
/* Globals: addresses not a multiple of their width are declared as byte
   arrays and accessed via __builtin_memcpy (packed). */
extern unsigned char DAT_00aa7b3e[4];   /* stored from param_3; width assumed 4 (unconfirmed) */
extern unsigned char DAT_00aa7b26[4];   /* HWND, unaligned */
extern char          DAT_00aa7b96;      /* width assumed 1 (unconfirmed) */
extern char          DAT_00aa7aa0;
extern char          DAT_00aa739e;
extern char          DAT_00aa739f;
extern char          DAT_00aa73a0;
extern char          DAT_00aa73a1;
extern unsigned char DAT_00aa7209[4];   /* uint, unaligned */
extern unsigned char DAT_00aa720d[4];   /* uint, unaligned */
extern unsigned int  DAT_00aa7598[0x40];
extern char          DAT_00aa7698[0x400];
extern unsigned int  DAT_00aa7a98;
extern unsigned int  DAT_00aa7a9c;

void __cdecl FUN_004d3350(void);
void __cdecl FUN_004d3400(void);
void __cdecl FUN_004d3820(int param_1);
void __cdecl FUN_004d5380(void);
void __cdecl FUN_004de150(void);

LRESULT __stdcall FUN_004d3a00(HWND param_1, uint param_2, uint param_3, uint param_4)
{
    unsigned int uVar2;
    unsigned int local_x;
    unsigned int local_y;
    int bVar1;
    HWND hCap;
    int iVar4;
    unsigned int *puVar5;

    if (param_2 < 0x1d) {
        if (param_2 == 0x1c) {
            __builtin_memcpy(DAT_00aa7b3e, &param_3, 4);
            if (param_3 == 0) {
                FUN_004d3350();
                ReleaseCapture();
            } else {
                FUN_004d3400();
                __builtin_memcpy(&hCap, DAT_00aa7b26, 4);
                if (hCap != (HWND)0) {
                    __builtin_memcpy(&hCap, DAT_00aa7b26, 4);
                    SetCapture(hCap);
                }
                DAT_00aa7b96 = 1;
                FUN_004de150();
            }
            goto LAB_004d3cd3;
        }
        if (param_2 == 2) {
            FUN_004d3820(1);
            PostQuitMessage(0);
            return 0;
        }
        goto LAB_004d3cd3;
    }
    if (param_2 < 0x101) {
        if (param_2 == 0x100) {
            if ((param_3 == 0x1b) && (DAT_00aa7aa0 != '\0')) {
                PostMessageA(param_1, 0x10, 0, 0);
            }
            goto LAB_004d3cd3;
        }
        if (param_2 == 0x20) {
            SetCursor((HCURSOR)0);
            return 1;
        }
        goto LAB_004d3cd3;
    }
    if (0x200 < param_2) {
        switch (param_2) {
        case 0x201:
            DAT_00aa739e = 1;
            if ((param_3 & 2) != 0) {
                DAT_00aa739f = 1;
            }
            break;
        case 0x202:
            DAT_00aa739e = 0;
            DAT_00aa73a0 = 1;
            if ((param_3 & 2) != 0) {
                DAT_00aa739f = 0;
                DAT_00aa73a1 = 1;
            }
            break;
        default:
            goto LAB_004d3cd3;
        case 0x204:
            DAT_00aa739f = 1;
            if ((param_3 & 1) != 0) {
                DAT_00aa739e = 1;
            }
            break;
        case 0x205:
            DAT_00aa739f = 0;
            DAT_00aa73a1 = 1;
            if ((param_3 & 1) != 0) {
                DAT_00aa739e = 0;
                DAT_00aa73a0 = 1;
            }
        }
        goto LAB_004d3cd3;
    }
    if (param_2 == 0x200) {
        uVar2 = param_4 & 0xffff;
        local_y = param_4 >> 0x10;
        local_x = uVar2;
        __builtin_memcpy(DAT_00aa720d, &local_y, 4);
        __builtin_memcpy(DAT_00aa7209, &local_x, 4);
        if (0x280 < uVar2) {
            local_x = 0x280;
            __builtin_memcpy(DAT_00aa7209, &local_x, 4);
        }
        bVar1 = 0x1e0 < local_y;
        if (bVar1) {
            local_y = 0x1e0;
            __builtin_memcpy(DAT_00aa720d, &local_y, 4);
        }
        if (bVar1 || 0x280 < uVar2) {
            __builtin_memcpy(&local_y, DAT_00aa720d, 4);
            __builtin_memcpy(&local_x, DAT_00aa7209, 4);
            SetCursorPos((int)local_x, (int)local_y);
        }
        goto LAB_004d3cd3;
    }
    switch (param_2) {
    case 0x101:
        puVar5 = DAT_00aa7598;
        for (iVar4 = 0x40; iVar4 != 0; iVar4 = iVar4 - 1) {
            *puVar5 = 0;
            puVar5 = puVar5 + 1;
        }
        goto LAB_004d3cd3;
    case 0x102:
        if (((DAT_00aa7a9c - DAT_00aa7a98 == 1) || ((DAT_00aa7a98 == 0x3ff) && (DAT_00aa7a9c == 0))) &&
            (DAT_00aa7a9c = DAT_00aa7a9c + 1, DAT_00aa7a9c == 0x400)) {
            DAT_00aa7a9c = 0;
        }
        DAT_00aa7698[DAT_00aa7a98] = (char)param_3;
        if (DAT_00aa7a98 == 0x3ff) {
            DAT_00aa7a98 = 0;
        } else {
            DAT_00aa7a98 = DAT_00aa7a98 + 1;
        }
        goto LAB_004d3cd3;
    default:
        goto LAB_004d3cd3;
    case 0x104:
        break;
    case 0x105:
        if (param_3 == 0x4f) {
            FUN_004d5380();
            return 0;
        }
        if (param_3 == 0x73) {
            return 0;
        }
    }
    if (param_3 == 0x73) {
        return 0;
    }
LAB_004d3cd3:
    return DefWindowProcA(param_1, param_2, param_3, param_4);
}
