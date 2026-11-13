/* SPDX-License-Identifier: MIT */
/* z_xvt 00425300 */
extern int DAT_006d49a0;
extern unsigned short DAT_0070a1e0;
extern unsigned short DAT_0071b020;

void FUN_0048f4b0(unsigned int);

void FUN_00425300(void)
{
  int iVar1;
  int iVar2;
  int iVar3;
  unsigned short *puVar4;
  int local_4;
  
  iVar1 = 0;
  if (0 < DAT_006d49a0) {
    puVar4 = &DAT_0070a1e0;
    do {
      iVar3 = 8;
      do {
        iVar2 = 3;
        do {
          if (*puVar4 != 0) {
            FUN_0048f4b0((unsigned int)*puVar4);
          }
          puVar4 = puVar4 + 1;
          iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
      iVar1 = iVar1 + 1;
    } while (iVar1 < DAT_006d49a0);
  }
  local_4 = 10;
  puVar4 = &DAT_0071b020;
  do {
    iVar1 = 7;
    do {
      iVar3 = 4;
      do {
        iVar2 = 3;
        do {
          if (*puVar4 != 0) {
            FUN_0048f4b0((unsigned int)*puVar4);
          }
          puVar4 = puVar4 + 1;
          iVar2 = iVar2 + -1;
        } while (iVar2 != 0);
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
    local_4 = local_4 + -1;
  } while (local_4 != 0);
  return;
}
