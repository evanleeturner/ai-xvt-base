/* SPDX-License-Identifier: MIT */
/* z_bop 004add60 */
// Prototypes for called functions
extern void * __cdecl FUN_004ac720(int);
extern void __cdecl FUN_00471ff0(int *);
extern int __cdecl FUN_004adcc0(int);
extern void __cdecl FUN_004ac740(void);

// Global placeholders
extern char DAT_00521f19[];
extern short DAT_009a7ed0[];
extern float DAT_00662ce8;
extern float DAT_00662cec;
extern float DAT_00663980;
extern float DAT_00662cf0;
extern float DAT_00663984;
extern int DAT_00663658;
extern float DAT_00663988;

void __cdecl FUN_004add60(int param_1)
{
  int *piVar1;
  int iVar2;
  float *pfVar3;
  int iVar4;
  int iVar5;
  float local_18;
  float local_14;
  float local_10;
  float local_c;
  float local_8;
  float local_4;
  int count_val;
  int array_base_val;

  local_10 = 1.0737418e+09f;
  local_14 = 1.0737418e+09f;
  local_18 = 1.0737418e+09f;
  local_4 = -1.0737418e+09f;
  local_8 = -1.0737418e+09f;
  local_c = -1.0737418e+09f;

  if ((DAT_00521f19[param_1 * 0x1a] & 1) != 0) {
    piVar1 = (int *)FUN_004ac720(((int)(short)((unsigned int)(param_1 * 0x19) >> 16) << 16) | (unsigned short)DAT_009a7ed0[param_1]);
    if ((int *)*piVar1 != piVar1) {
      FUN_00471ff0(piVar1);
    }
    __builtin_memcpy(&count_val, (void *)((int)piVar1 + 6), 4);
    __builtin_memcpy(&array_base_val, (void *)((int)piVar1 + 10), 4);
    iVar4 = 0;
    iVar5 = 0;
    if (0 < count_val) {
      do {
        iVar2 = *(int *)(array_base_val + iVar4);
        if ((iVar2 != 0) && (*(int *)(iVar2 + 4) != 0x14)) {
          iVar2 = FUN_004adcc0(iVar2);
          if (iVar2 != 0) {
            if (1 < *(int *)(iVar2 + 0x10)) {
              pfVar3 = (float *)(*(int *)(iVar2 + 0x14) + *(int *)(iVar2 + 0x10) * 0xc + -0x18);
              if (*pfVar3 < local_18) {
                local_18 = *pfVar3;
              }
              if (pfVar3[1] < local_14) {
                local_14 = pfVar3[1];
              }
              if (pfVar3[2] < local_10) {
                local_10 = pfVar3[2];
              }
              if (local_c < pfVar3[3]) {
                local_c = pfVar3[3];
              }
              if (local_8 < pfVar3[4]) {
                local_8 = pfVar3[4];
              }
              if (local_4 < pfVar3[5]) {
                local_4 = pfVar3[5];
              }
            }
          }
        }
        iVar4 = iVar4 + 4;
        iVar5 = iVar5 + 1;
      } while (iVar5 < count_val);
    }
    *(float *)((char *)&DAT_00662ce8 + param_1 * 0xc) = local_18;
    iVar4 = param_1 * 0xc;
    *(float *)((char *)&DAT_00662cec + iVar4) = local_14;
    *(float *)((char *)&DAT_00663980 + iVar4) = local_c;
    *(float *)((char *)&DAT_00662cf0 + iVar4) = local_10;
    *(float *)((char *)&DAT_00663984 + iVar4) = local_8;
    *(int *)((char *)&DAT_00663658 + param_1 * 4) = 1;
    *(float *)((char *)&DAT_00663988 + iVar4) = local_4;
    FUN_004ac740();
  }
  return;
}
