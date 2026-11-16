/* SPDX-License-Identifier: MIT */
/* z_xvt 00426e50 */
extern int *DAT_00536ecc;
extern int DAT_00536ed0;
extern char DAT_00524a18[];
extern unsigned int DAT_00524e1c;
extern unsigned int DAT_00524e18;
extern unsigned int DAT_00524e20;
extern unsigned int unaff_EDI;

int __stdcall FUN_00426e50(void)
{
  char cVar1;
  int iVar2;
  unsigned int uVar3;
  int *puStack_38;
  unsigned int uStack_34;
  int local_24;
  char local_20[32];
  
  while (1) {
    while (1) {
      puStack_38 = &local_24;
      uStack_34 = 1;
      local_24 = 1;
      iVar2 = ((int (__stdcall *)(int *, int, char *))(*(int **)DAT_00536ecc + 0xA))(DAT_00536ecc, 0x10, local_20);
      if (iVar2 != -0x7ff8ffe2) break;
      if (DAT_00536ecc == NULL) {
        return 0;
      }
      ((void (__stdcall *)(int *))(*(int **)DAT_00536ecc + 0x7))(DAT_00536ecc);
      DAT_00536ed0 = 1;
    }
    if (iVar2 < 0) {
      return 0;
    }
    if (puStack_38 == NULL) break;
    uVar3 = unaff_EDI & 0x80;
    cVar1 = DAT_00524a18[uStack_34 & 0xff];
    if (cVar1 == -3) {
      puStack_38 = (int *)1;
      DAT_00524e1c = uVar3;
      ((void (__stdcall *)(int *, int, unsigned int *, int **, int))(*(int **)DAT_00536ecc + 0xA))(DAT_00536ecc, 0x10, &uStack_34, &puStack_38, 0);
    }
    else if (cVar1 == -2) {
      puStack_38 = (int *)1;
      DAT_00524e18 = uVar3;
      ((void (__stdcall *)(int *, int, unsigned int *, int **, int))(*(int **)DAT_00536ecc + 0xA))(DAT_00536ecc, 0x10, &uStack_34, &puStack_38, 0);
    }
    else if (cVar1 == -1) {
      puStack_38 = (int *)1;
      DAT_00524e20 = uVar3;
      ((void (__stdcall *)(int *, int, unsigned int *, int **, int))(*(int **)DAT_00536ecc + 0xA))(DAT_00536ecc, 0x10, &uStack_34, &puStack_38, 0);
    }
    else {
      if (uVar3 != 0) {
        return 1;
      }
      puStack_38 = (int *)1;
      ((void (__stdcall *)(int *, int, unsigned int *, int **, int))(*(int **)DAT_00536ecc + 0xA))(DAT_00536ecc, 0x10, &uStack_34, &puStack_38, 0);
    }
  }
  return 0;
}
