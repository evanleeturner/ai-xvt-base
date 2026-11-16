/* SPDX-License-Identifier: MIT */
/* z_bop 004830d0 */
extern int DAT_009a1fe8;
extern int DAT_009d8a58;
extern int DAT_009d8a54;
extern int DAT_009d8b60;
extern int DAT_00a07cdc;
extern int DAT_00a07cd8;
extern int DAT_00a07cd4;
extern char DAT_009e9a5b;
extern char DAT_009e9a5f;
extern char DAT_009e9a63;
extern int DAT_009ecc34;

void FUN_00458e20(unsigned int, int);
int FUN_004ae640(unsigned int, int);
int FUN_004ae6e0(unsigned int, int);
int FUN_004ae5a0(unsigned int, int);
void FUN_004037b0(int, int, int, int);
unsigned int FUN_00427500(int, int, int);
unsigned int FUN_00427420(int, int, int);
unsigned int FUN_00427490(int, int, int);
unsigned int FUN_00427570(unsigned int, unsigned int);
unsigned int FUN_00427590(unsigned int, unsigned int);

void FUN_004830d0(unsigned int param_1, int param_2, unsigned int *param_3, unsigned int *param_4, unsigned int *param_5)
{
  int iVar1;
  int iVar2;
  int iVar3;
  unsigned int uVar4;
  unsigned int uVar5;
  unsigned int uVar6;
  unsigned int uVar7;
  int iVar8;
  int local_9e9a5b;
  int local_9e9a5f;
  int local_9e9a63;

  FUN_00458e20(param_1, 0);
  if (param_2 != 0xffff) {
    iVar8 = param_1 * 0x23 + DAT_009a1fe8;
    uVar4 = (unsigned int)*(unsigned char *)(iVar8 + 3);
    iVar1 = FUN_004ae640(uVar4, param_2);
    iVar1 = -iVar1;
    iVar2 = FUN_004ae6e0(uVar4, param_2);
    iVar3 = FUN_004ae5a0(uVar4, param_2);
    FUN_004037b0(iVar8, iVar3, iVar2, iVar1);
    DAT_00a07cdc = DAT_00a07cdc + DAT_009d8a58;
    DAT_00a07cd8 = DAT_00a07cd8 + DAT_009d8a54;
    DAT_00a07cd4 = DAT_00a07cd4 + DAT_009d8b60;
  }
  __builtin_memcpy(&local_9e9a5b, (char *)&DAT_009e9a5b + DAT_009ecc34 * 0x5bd, 4);
  __builtin_memcpy(&local_9e9a5f, (char *)&DAT_009e9a5f + DAT_009ecc34 * 0x5bd, 4);
  __builtin_memcpy(&local_9e9a63, (char *)&DAT_009e9a63 + DAT_009ecc34 * 0x5bd, 4);
  iVar3 = DAT_00a07cdc - local_9e9a5b;
  iVar1 = DAT_00a07cd8 - local_9e9a5f;
  iVar2 = DAT_00a07cd4 - local_9e9a63;
  uVar4 = FUN_00427500(iVar3, iVar1, iVar2);
  *param_5 = uVar4;
  if (0 < (int)uVar4) {
    uVar5 = FUN_00427420(iVar3, iVar1, iVar2);
    uVar6 = FUN_00427490(iVar3, iVar1, iVar2);
    uVar7 = FUN_00427570(uVar5, uVar4);
    *param_3 = uVar7;
    uVar7 = FUN_00427590(uVar6, uVar4);
    *param_4 = uVar7;
  }
  return;
}
