/* SPDX-License-Identifier: MIT */
/* z_bop 004f7480 */
extern short DAT_0066925a;
extern short DAT_0066925c;
extern short DAT_00669264[];
extern short DAT_00669262[];
extern short DAT_006696e2;
extern char DAT_006696da[];
extern short DAT_00669778;
extern int DAT_00669218;
extern int DAT_0066921c;

void __stdcall FUN_004f7380(void);
void __cdecl FUN_004f7590(int);
void __cdecl FUN_004f7420(short, int);

void __stdcall FUN_004f7480(void)
{
  short sVar1;
  int bVar2;
  int bVar3;
  short sVar4;
  short sVar5;
  short sVar6;
  
  sVar1 = DAT_0066925a;
  bVar2 = 0;
  sVar6 = 0;
  sVar5 = 0;
  bVar3 = 0;
  FUN_004f7380();
  do {
    if (sVar5 == 0x22) break;
    sVar5 = DAT_00669264[DAT_0066925c];
    if (DAT_006696e2 != 0) {
      sVar6 = 0;
      bVar2 = 0;
    }
    sVar4 = 0;
    do {
      if (DAT_006696da[sVar4] != 0) {
        bVar2 = 1;
      }
      sVar4 = sVar4 + 1;
    } while (sVar4 < 2);
    if (bVar2) {
      sVar6 = sVar6 + 1;
    }
    if (((DAT_00669778 == 0) && (sVar6 != 1)) || (DAT_0066925a < sVar1)) {
      FUN_004f7590(1);
    }
    else {
      bVar3 = 1;
    }
  } while (!bVar3);
  if ((DAT_00669778 == 0) && (sVar6 != 1)) {
    sVar5 = DAT_00669264[DAT_0066925c];
    sVar1 = DAT_00669262[DAT_0066925c];
  }
  else {
    sVar5 = 0;
    sVar1 = DAT_0066925a;
  }
  if (sVar5 == 0x22) {
    DAT_00669218 = 0;
    DAT_0066921c = 0;
    FUN_004f7380();
    return;
  }
  FUN_004f7420(sVar1, 0);
  return;
}
