/* SPDX-License-Identifier: MIT */
/* z_bop 00437b20 */
extern unsigned char DAT_009e9a5b;
extern unsigned char DAT_009e9a5f;
extern unsigned char DAT_009e9a63;
extern int DAT_009ecc34;
extern int DAT_00a07cd4;
extern int DAT_00a07cd8;
extern int DAT_00a07cdc;
extern int DAT_009fd39c;
extern int DAT_009fd3a0;
extern int DAT_009fd398;
extern unsigned int DAT_009a8e34;
extern unsigned int DAT_009a8e2c;
extern unsigned int DAT_009a8e30;
extern unsigned int DAT_00a004b0;
extern unsigned int DAT_00a004a8;
extern unsigned int DAT_00a004a4;
extern unsigned int DAT_009fe7f4;
extern unsigned int DAT_00a004cc;
extern unsigned int DAT_00a004b8;
extern short DAT_00a0813c;
extern short DAT_009a6fe0;
extern void (*DAT_009cc454)(int, int, int, int, unsigned int);

unsigned int FUN_00427500(int a, int b, int c);
unsigned int FUN_00427420(int a, int b, int c);
unsigned int FUN_00427490(int a, int b, int c);
void FUN_00427390(unsigned int a, unsigned int b, unsigned int c);
void FUN_004aba50(void);
int FUN_00427590(unsigned int a, unsigned int b);
int FUN_00427570(unsigned int a, unsigned int b);
void FUN_004abe50(void);

void __stdcall FUN_00437b20(void)
{
  unsigned int uVar1;
  unsigned int uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  unsigned int uVar7;
  int iVar8;
  int iVar9;
  unsigned int uVar10;
  unsigned int uVar11;
  unsigned int uVar12;
  unsigned int uVar13;
  int local_c;
  int local_8;
  int local_4;
  int tmp;

  DAT_00a07cd4 = 0;
  DAT_00a07cd8 = 0;
  DAT_00a07cdc = 0;
  local_8 = -0x100000;
  local_c = 0x21;
  do {
    __builtin_memcpy(&tmp, &DAT_009e9a5b + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd39c = (int)(0xfff00000u - (unsigned int)tmp);
    __builtin_memcpy(&tmp, &DAT_009e9a5f + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd3a0 = (int)((unsigned int)local_8 - (unsigned int)tmp);
    __builtin_memcpy(&tmp, &DAT_009e9a63 + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd398 = (int)(0xffff0000u - (unsigned int)tmp);
    local_8 = (int)((unsigned int)local_8 + 0x10000u);
    DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    uVar11 = DAT_00a004b0 * 0x40u + DAT_009a8e34;
    if ((0 < (int)uVar11) || (0 < (int)DAT_009a8e34)) {
      DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
      DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
      uVar1 = DAT_00a004a8 * 0x40u + DAT_009a8e2c;
      uVar2 = DAT_00a004a4 * 0x40u + DAT_009a8e30;
      uVar7 = DAT_009a8e34;
      uVar10 = uVar2;
      uVar12 = uVar1;
      if ((int)uVar11 < 1) {
        uVar7 = uVar11;
        uVar10 = DAT_009a8e30;
        uVar11 = DAT_009a8e34;
        uVar12 = DAT_009a8e2c;
        DAT_009a8e2c = uVar1;
        DAT_009a8e30 = uVar2;
      }
      DAT_009a8e34 = uVar7;
      if ((int)uVar7 < 1) {
        FUN_00427390(uVar12, uVar10, uVar11);
      }
      FUN_004aba50();
      iVar8 = (int)DAT_00a0813c;
      iVar3 = (int)DAT_009a6fe0;
      uVar13 = 0x31;
      iVar4 = FUN_00427590(uVar10, uVar11);
      iVar4 = (int)((unsigned int)iVar4 + (unsigned int)iVar8);
      iVar5 = FUN_00427570(uVar12, uVar11);
      uVar11 = DAT_009a8e34;
      iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar3);
      iVar6 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
      iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar8);
      iVar8 = FUN_00427570(DAT_009a8e2c, uVar11);
      (*DAT_009cc454)((int)((unsigned int)iVar8 + (unsigned int)iVar3), iVar6, iVar5, iVar4, uVar13);
      FUN_004abe50();
    }
    local_c = local_c + -1;
  } while (local_c != 0);
  local_c = 0x21;
  iVar3 = (int)((unsigned int)DAT_00a07cd8 - 0x100000u);
  local_4 = (int)((unsigned int)DAT_00a07cdc - 0x100000u);
  do {
    __builtin_memcpy(&tmp, &DAT_009e9a63 + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd398 = (int)(0xffff0000u - (unsigned int)tmp);
    __builtin_memcpy(&tmp, &DAT_009e9a5b + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd39c = (int)((unsigned int)local_4 - (unsigned int)tmp);
    __builtin_memcpy(&tmp, &DAT_009e9a5f + (int)((unsigned int)DAT_009ecc34 * 0x5bdu), 4);
    DAT_009fd3a0 = (int)((unsigned int)iVar3 - (unsigned int)tmp);
    DAT_009a8e34 = FUN_00427500(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
    uVar11 = DAT_009fe7f4 * 0x40u + DAT_009a8e34;
    if ((0 < (int)uVar11) || (0 < (int)DAT_009a8e34)) {
      DAT_009a8e2c = FUN_00427420(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
      DAT_009a8e30 = FUN_00427490(DAT_009fd39c, DAT_009fd3a0, DAT_009fd398);
      uVar1 = DAT_00a004cc * 0x40u + DAT_009a8e2c;
      uVar2 = DAT_00a004b8 * 0x40u + DAT_009a8e30;
      uVar7 = DAT_009a8e34;
      uVar10 = uVar2;
      uVar12 = uVar1;
      if ((int)uVar11 < 1) {
        uVar7 = uVar11;
        uVar10 = DAT_009a8e30;
        uVar11 = DAT_009a8e34;
        uVar12 = DAT_009a8e2c;
        DAT_009a8e2c = uVar1;
        DAT_009a8e30 = uVar2;
      }
      DAT_009a8e34 = uVar7;
      if ((int)uVar7 < 1) {
        FUN_00427390(uVar12, uVar10, uVar11);
      }
      FUN_004aba50();
      iVar9 = (int)DAT_00a0813c;
      iVar4 = (int)DAT_009a6fe0;
      uVar13 = 0x31;
      iVar5 = FUN_00427590(uVar10, uVar11);
      iVar5 = (int)((unsigned int)iVar5 + (unsigned int)iVar9);
      iVar6 = FUN_00427570(uVar12, uVar11);
      uVar11 = DAT_009a8e34;
      iVar6 = (int)((unsigned int)iVar6 + (unsigned int)iVar4);
      iVar8 = FUN_00427590(DAT_009a8e30, DAT_009a8e34);
      iVar8 = (int)((unsigned int)iVar8 + (unsigned int)iVar9);
      iVar9 = FUN_00427570(DAT_009a8e2c, uVar11);
      (*DAT_009cc454)((int)((unsigned int)iVar9 + (unsigned int)iVar4), iVar8, iVar6, iVar5, uVar13);
      FUN_004abe50();
    }
    local_c = local_c + -1;
    local_4 = (int)((unsigned int)local_4 + 0x10000u);
  } while (local_c != 0);
  return;
}
