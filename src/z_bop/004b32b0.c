/* SPDX-License-Identifier: MIT */
/* z_bop 004b32b0 */
int __cdecl FUN_004b1600(int param_1, int *param_2, int param_3);
void __stdcall FUN_004079e0(void);

extern int DAT_00a90b20;
extern int DAT_00a90bf0;
extern int DAT_00a90bf4;
extern int DAT_00a90bf8;
extern int DAT_00a90bfc;
extern void **DAT_00a91a34;
extern int DAT_00528cc0;

int __stdcall FUN_004b32b0(void)
{
  int iVar1;
  int local_74;
  int local_70;
  int local_6c;
  int local_68;
  int local_64[0x19];
  int local_14;
  int i;

  for (i = 0; i < 0x19; i++) {
    local_64[i] = 0;
  }
  local_64[0] = 100;
  local_14 = 0;
  if (DAT_00a90b20 != 0x10) {
    local_14 = 0xffff;
  }
  local_74 = DAT_00a90bf0;
  local_70 = DAT_00a90bf4;
  local_6c = DAT_00a90bf8 + DAT_00a90bf0;
  local_68 = DAT_00a90bfc + DAT_00a90bf4;
  do {
    iVar1 = (*(int (__stdcall **)(void *, int *, int, int, int, int *))((char *)*DAT_00a91a34 + 0x14))(DAT_00a91a34, &local_74, 0, 0, 0x3000000, local_64);
    if (iVar1 == 0) {
      return 1;
    }
    if (iVar1 == -0x7789fe3e) {
      iVar1 = (*(int (__stdcall **)(void *))((char *)*DAT_00a91a34 + 0x6c))(DAT_00a91a34);
    }
  } while (iVar1 == 0);
  FUN_004b1600(iVar1, (int *)&DAT_00528cc0, 0x79);
  FUN_004079e0();
  (void)local_70;
  (void)local_6c;
  (void)local_68;
  (void)local_14;
  return 0;
}
