/* SPDX-License-Identifier: MIT */
/* z_bop 004df5f0 */
extern int DAT_00b6a5aa;
extern int DAT_00b6a606;
extern int DAT_00b6a248;
extern int DAT_00b69e34;
extern int DAT_00b6a5c2;
extern int DAT_00b6a5ae;
extern int DAT_00aa6cfc;

void __cdecl FUN_004d3350(void);
void __cdecl FUN_004d4f70(void);
void __cdecl FUN_004d4970(void);
void __cdecl FUN_004d4c10(void);
void __cdecl FUN_004d49a0(void);
int __cdecl FUN_004efce0(char *param_1, int param_2);
void __cdecl FUN_004d4f80(void);
int __cdecl FUN_004d48e0(void);
void __cdecl FUN_004d4f60(void);
void __cdecl FUN_004d3400(void);

int __cdecl FUN_004df5f0(int param_1)
{
  unsigned char bVar1;
  int iVar2;
  int iVar3;
  unsigned int uVar4;

  if ((DAT_00b6a5aa == 0) && (DAT_00b6a606 == 1)) {
    if (DAT_00b6a248 == 0) {
      return 0;
    }
    uVar4 = 0;
    if (DAT_00b69e34 != 0) {
      iVar3 = 0;
      do {
        iVar2 = DAT_00b6a248 + iVar3;
        if (((*(int *)(iVar2 + 0x120) == DAT_00b6a5c2) && (*(int *)(iVar2 + 0x124) == param_1)) &&
           (*(int *)(iVar2 + 0x128) == DAT_00b6a5ae)) {
          FUN_004d3350();
          FUN_004d4f70();
          FUN_004d4970();
          FUN_004d4c10();
          FUN_004d49a0();
          FUN_004d4c10();
          bVar1 = FUN_004efce0((char *)(DAT_00b6a248 + iVar3), 1);
          FUN_004d4c10();
          FUN_004d49a0();
          FUN_004d4c10();
          FUN_004d4f80();
          DAT_00aa6cfc = FUN_004d48e0();
          FUN_004d4f60();
          FUN_004d3400();
          if (bVar1 != 0) {
            return 0;
          }
        }
        iVar3 = iVar3 + 300;
        uVar4 = uVar4 + 1;
      } while (uVar4 < (unsigned int)DAT_00b69e34);
    }
    return 1;
  }
  return 0;
}
