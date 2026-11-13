/* SPDX-License-Identifier: MIT */
/* z_xvt 004b15a0 */
extern int DAT_00656c90;
extern int DAT_00655ab4;
extern int DAT_00655ab0;
extern int DAT_00655ab2;
extern int DAT_0071f2e2;
extern int DAT_006d4a4e;
extern void (__cdecl *DAT_00703c80)(void);
extern int DAT_0071b730;
extern int DAT_0071ecf4;
extern int DAT_006d4988;
extern int DAT_00655ab6;

void __cdecl FUN_0048ed00(short, short, short, short);
void __cdecl FUN_0048ed70(uint);
void __cdecl FUN_0048ece0(ushort, ushort);
void __cdecl FUN_0040ccc0(ushort, uint, uint);

void __cdecl FUN_004b15a0(ushort param_1, ushort param_2, uint param_3)
{
  uint uVar3;
  ushort uVar2;
  int iVar1;

  uVar3 = (uint)param_1;
  if (*(ushort *)(&DAT_00656c90 + uVar3 * 2) != param_2) {
    *(ushort *)(&DAT_00656c90 + uVar3 * 2) = param_2;
    uVar2 = *(ushort *)(&DAT_00655ab4 + uVar3 * 0xc);
    iVar1 = uVar3 * 0xc;
    FUN_0048ed00(*(short *)(&DAT_00655ab0 + iVar1),
                *(short *)(&DAT_00655ab2 + uVar3 * 0xc),
                DAT_0071f2e2 * uVar2 + *(short *)(&DAT_00655ab0 + iVar1) + 2,
                (ushort)DAT_006d4a4e + *(short *)(&DAT_00655ab2 + uVar3 * 0xc));
    (*DAT_00703c80)();
    if (0x78 < param_1) {
      param_1 = param_1 + ((ushort)(param_1 - 1) / 0x78) * -0x78;
    }
    if ((param_2 < 0x15) &&
        (((param_1 == 0x3d || param_1 == 0x3a) || param_1 == 0x4e) ||
         (param_1 == 0x3e || param_1 == 0x4f))) {
      uVar3 = 0x4a;
    }
    else if ((param_2 < 0x33) &&
             ((param_1 == 0x3d || param_1 == 0x3a) ||
              (param_1 == 0x3e || (param_1 == 0x4e || param_1 == 0x4f)))) {
      uVar3 = 0x4e;
    }
    else if ((param_1 == 0x19 || param_1 == 0x18) &&
             (*(short *)(*(int *)(*(int *)(*(int *)((int)&DAT_0071b730 + DAT_0071ecf4 * 0x5bd) * 0x23
                                           + 0x1f + DAT_006d4988) + 0xa7) + 0xbf) == 0)) {
      uVar3 = 0x52;
    }
    else {
      uVar3 = (uint)*(ushort *)(&DAT_00655ab6 + iVar1);
    }
    FUN_0048ed70(uVar3);
    FUN_0048ece0(*(ushort *)(&DAT_00655ab0 + iVar1), *(ushort *)(&DAT_00655ab2 + iVar1));
    FUN_0040ccc0(param_2, (uint)uVar2, param_3 & 0xffff);
  }
  return;
}
