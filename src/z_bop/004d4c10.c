/* SPDX-License-Identifier: MIT */
/* z_bop 004d4c10 */
extern char DAT_00aa7b0d;
extern int DAT_00aa7b2e;
extern int *DAT_00aa7b3a;
extern int DAT_00aa7b0f;
extern int DAT_00aa6cfc;

void __stdcall FUN_004d4970(void);
void __cdecl FUN_004d6460(int *param_1, int param_2, int param_3, int param_4, int param_5);
int __stdcall FUN_004d37e0(void);
int __stdcall FUN_004d48e0(void);

void __stdcall FUN_004d4c10(void)
{
  char cVar1;
  int iVar2;
  int *puVar3;
  int local_74[4];
  int local_64[20];
  
  cVar1 = DAT_00aa7b0d;
  if ((DAT_00aa7b2e != 0) && (DAT_00aa7b3a != 0)) {
    FUN_004d4970();
    FUN_004d6460(local_74, 0, 0, 0x280, 0x1e0);
    puVar3 = local_64;
    for (iVar2 = 0x19; iVar2 != 0; iVar2 = iVar2 - 1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    local_64[0] = 100;
    do {
      while (1) {
        iVar2 = ((int (__stdcall *)(int *, int *, int, int, int, int *))(*(int **)DAT_00aa7b3a + 0x14))(DAT_00aa7b3a, local_74, 0, 0, 0x400, local_64);
        if (iVar2 == 0) {
          if (cVar1 == '\0') {
            return;
          }
          DAT_00aa6cfc = FUN_004d48e0();
          return;
        }
        if (iVar2 != -0x7789fe3e) break;
        iVar2 = FUN_004d37e0();
        if (iVar2 != 0) {
          if (cVar1 == '\0') {
            return;
          }
          DAT_00aa6cfc = FUN_004d48e0();
          return;
        }
      }
    } while (iVar2 == -0x7789fde4);
    if (cVar1 != '\0') {
      DAT_00aa6cfc = FUN_004d48e0();
    }
  }
  return;
}
