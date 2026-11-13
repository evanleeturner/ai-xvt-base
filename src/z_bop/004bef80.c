/* SPDX-License-Identifier: MIT */
/* z_bop 004bef80 */
extern int DAT_00b69e28;
extern int DAT_00b69cc0;
extern int DAT_00b6a2a0;
extern unsigned int DAT_00b6a2c0;
extern int DAT_00b6a2e0;

extern void FUN_004d6480(int *, int *);
extern void FUN_004de090(int *, int *);
extern int FUN_004d6b90(int *, int, int);
extern void FUN_004d6550(int *, int, int, int);
extern int __stdcall FUN_004dc220(void);
extern int __stdcall FUN_004dc240(void);
extern void FUN_004d4cf0(int *);
extern void FUN_004d4d20(int *);
extern int __strcmpi(const char *, const char *);
extern void FUN_004db7d0(int, char *, int *, int, int, unsigned int);
extern void FUN_004d64a0(int *, int, int);

int __cdecl FUN_004bef80(int *param_1, int param_2)
{
  int uVar1;
  int iVar2;
  char *pcVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  unsigned int uVar7;
  int local_2c;
  int local_28 = 0;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10[4];

  if (DAT_00b69e28 == 0) {
    return 0;
  }
  FUN_004d6480(&local_2c, param_1);
  FUN_004de090(&local_14, &local_18);
  local_20 = local_28 + 0xe;
  (void)local_20;
  iVar4 = 0;
  local_1c = *(int *)((char *)DAT_00b69e28 + 4);
  if ((DAT_00b69cc0 != 0) && (param_2 < param_2 + 0xd)) {
    iVar5 = param_2 * 0xe;
    iVar6 = param_2;
    do {
      if (local_1c <= iVar6) {
        return iVar4;
      }
      iVar2 = FUN_004d6b90(&local_2c, local_14, local_18);
      if (iVar2 != 0) {
        FUN_004d6550(&local_2c, 0, 0, DAT_00b6a2a0);
        uVar1 = FUN_004dc220();
        if ((uVar1 != 0) || (uVar1 = FUN_004dc240(), uVar1 != 0)) {
          iVar4 = iVar6 + 1;
        }
      }
      FUN_004d4cf0(local_10);
      FUN_004d4d20(&local_2c);
      iVar2 = __strcmpi((char *)DAT_00b69cc0 + iVar5, (char *)&DAT_00b6a2e0);
      uVar7 = DAT_00b6a2c0;
      if (iVar2 != 0) {
        uVar7 = 0xffff;
      }
      iVar6 = iVar6 + 1;
      pcVar3 = (char *)DAT_00b69cc0 + iVar5;
      iVar5 = iVar5 + 0xe;
      FUN_004db7d0(0xc, pcVar3, &local_2c, 0, 1, uVar7);
      FUN_004d4d20(local_10);
      FUN_004d64a0(&local_2c, 0, 0xf);
    } while (iVar6 < param_2 + 0xd);
  }
  return iVar4;
}
