/* SPDX-License-Identifier: MIT */
/* z_xvt 0051a3d0 */
extern int __cdecl FUN_004d5500(void);
extern unsigned short DAT_00742460[];
extern void (__cdecl *DAT_00742814)(int, int, int);
extern int DAT_005366fc;

void __cdecl FUN_0051a3d0(int param_1, int param_2, int param_3)
{
  unsigned char bVar1;
  unsigned char bVar2;
  unsigned char bVar3;
  unsigned char *pbVar5;
  unsigned short uVar6;
  unsigned short *local_8;
  int local_4;
  int ret;

  if (param_2 < param_2 + param_3) {
    pbVar5 = (unsigned char *)(param_1 + param_2 * 3);
    local_4 = param_3;
    local_8 = &DAT_00742460[param_2];
    do {
      bVar1 = pbVar5[0];
      bVar2 = pbVar5[1];
      bVar3 = pbVar5[2];
      ret = FUN_004d5500();
      if (ret == 0) {
        uVar6 = (unsigned short)(bVar3 >> 3) | ((unsigned short)(bVar2 >> 2) << 5) | (((bVar1 >> 2) & 0xfffe) << 10);
      }
      else {
        uVar6 = (unsigned short)(bVar3 >> 3) | (((unsigned short)(bVar2 >> 2) & 0xfffe) << 4) | (((bVar1 >> 2) & 0xfffe) << 9);
      }
      pbVar5 += 3;
      *local_8 = uVar6;
      local_4--;
      local_8++;
    } while (local_4 != 0);
  }
  if (DAT_00742814 != NULL && DAT_005366fc == 1) {
    DAT_00742814(param_1, param_2, param_3);
  }
  return;
}
