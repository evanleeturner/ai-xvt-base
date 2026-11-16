/* SPDX-License-Identifier: MIT */
/* z_xvt 004cc820 */
extern char DAT_00759645;
extern int DAT_007806fa;
extern int DAT_008050aa;
extern int DAT_008050ae;
extern int DAT_00758830;
extern int DAT_007810a6;
extern int DAT_007810b6;

void __cdecl FUN_004d4d90(void);
void __cdecl FUN_004ca990(void);
int __cdecl FUN_004d4d00(void);

int __cdecl FUN_004cc820(int param_1)
{
  int iVar1;
  char cVar2;
  int iVar3;
  
  cVar2 = DAT_00759645;
  if (DAT_007806fa == 0) {
    return 0;
  }
  FUN_004d4d90();
  FUN_004ca990();
  iVar3 = DAT_008050aa;
  iVar1 = DAT_008050ae;
  if (DAT_008050ae < 0x201) {
    while (1) {
      if (iVar1 < 1) {
        if (cVar2 != '\0') {
          DAT_00758830 = FUN_004d4d00();
        }
        return 0;
      }
      if (((&DAT_007810a6)[iVar3 * 0x84] != 0) && ((&DAT_007810b6)[iVar3 * 0x84] == param_1)) break;
      iVar3 = iVar3 + 1;
      if (0x3ff < iVar3) {
        iVar3 = 0;
      }
      iVar1 = iVar1 + -1;
    }
    if (cVar2 != '\0') {
      DAT_00758830 = FUN_004d4d00();
    }
    return 1;
  }
  if (cVar2 != '\0') {
    DAT_00758830 = FUN_004d4d00();
  }
  return 1;
}
