/* SPDX-License-Identifier: MIT */
/* z_xvt 004db3f0 */
extern char DAT_00523570;
extern char DAT_00759bd8[];
extern char DAT_0075a1e0[];
extern int DAT_0075a1dc[];
extern void *DAT_0075d846[];

void * __cdecl FUN_004be160(char *param_1, char *param_2);
int __cdecl FUN_004be320(void *param_1, void *param_2, size_t param_3);
void * __cdecl _malloc(size_t param_1);
int __cdecl FUN_004be1f0(void *param_1);

int __cdecl FUN_004db3f0(char *param_1, int param_2)
{
  size_t *psVar1;
  size_t _Size;
  int iVar2;
  void *pFVar3;
  void *pvVar4;
  
  iVar2 = param_2 * 0x60b;
  psVar1 = (size_t *)((int)&DAT_00759bd8 + iVar2);
  pFVar3 = FUN_004be160(param_1, &DAT_00523570);
  if (pFVar3 == (void *)0x0) {
    return 0;
  }
  FUN_004be320(pFVar3, psVar1, 0x60b);
  _Size = *psVar1;
  pvVar4 = _malloc(_Size);
  *psVar1 = (size_t)pvVar4;
  if (pvVar4 == (void *)0x0) {
    DAT_0075a1e0[iVar2] = 0;
    FUN_004be1f0(pFVar3);
    return 0;
  }
  FUN_004be320(pFVar3, pvVar4, _Size);
  DAT_0075d846[DAT_0075a1dc[iVar2]] = psVar1;
  FUN_004be1f0(pFVar3);
  return 1;
}
