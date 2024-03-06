#include <stdio.h>

int main(int argc, char **argv)
{
  printf("Main file called with args: \n");
  for (int i = 0; i < argc; i++)
  {
    printf("%d. %s\n", i + 1, argv[i]);
  }
  return 0;
}